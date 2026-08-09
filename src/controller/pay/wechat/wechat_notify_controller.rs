use crate::{
    common::db::database::get_conn,
    composite::user::user_product_sub_handler::product_pay_success,
    model::diesel::{
        custom::pay::payment_add::PaymentAdd, dolphin::custom_dolphin_models::AppMap,
    },
    service::{
        app::app_map_service::query_app_maps_by_pay_type,
        order::order_service::query_order_by_out_trans_no,
        pay::sys::payment_service::save_payment,
    },
};
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use bigdecimal::{BigDecimal, ToPrimitive};
use diesel::Connection;
use labrador::{EncryptV3, WechatCryptoV3, WechatPayClient};
use log::{error, warn};
use rust_wheel::model::enums::{rd_pay_status::RdPayStatus, rd_pay_type::RdPayType};
use serde::{Deserialize, Serialize};

fn header_value(req: &HttpRequest, key: &str) -> String {
    req.headers()
        .get(key)
        .map(|v| v.to_str().unwrap_or_default().to_string())
        .unwrap_or_default()
}

/// Wechat V3 transaction notification envelope
#[derive(Deserialize)]
struct WechatNotifyEnvelope {
    #[allow(dead_code)]
    id: String,
    #[serde(rename = "event_type")]
    event_type: String,
    resource: EncryptV3,
}

/// Decrypted Wechat transaction plaintext
#[derive(Deserialize)]
struct WechatTransaction {
    #[allow(dead_code)]
    mchid: String,
    appid: String,
    out_trade_no: String,
    transaction_id: String,
    trade_state: String,
    amount: Option<WechatNotifyAmount>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct WechatNotifyAmount {
    total: i64,
    currency: Option<String>,
    payer_total: Option<i64>,
}

#[derive(Serialize)]
struct WechatNotifyResponse {
    code: String,
    message: String,
}

impl WechatNotifyResponse {
    fn ok() -> Self {
        WechatNotifyResponse {
            code: "SUCCESS".to_string(),
            message: "成功".to_string(),
        }
    }
}

fn build_client(app_map: &AppMap) -> WechatPayClient<labrador::SimpleStorage> {
    let secret = app_map.app_secret.clone().unwrap_or_default();
    WechatPayClient::<labrador::SimpleStorage>::new(&app_map.third_app_id, &secret)
        .key_v3(secret.clone())
        .mch_id(app_map.mch_id.clone())
        .serial_no(app_map.serial_no.clone())
        .private_key(app_map.app_private_key.clone())
}

#[post("/v1/wechatPayNotify")]
pub async fn wechat_pay_notify(req: HttpRequest, body: web::Bytes) -> impl Responder {
    let body_str = String::from_utf8_lossy(&body).to_string();
    let header = labrador::SignatureHeader {
        time_stamp: header_value(&req, "Wechatpay-Timestamp"),
        nonce: header_value(&req, "Wechatpay-Nonce"),
        signature: header_value(&req, "Wechatpay-Signature"),
        serial: header_value(&req, "Wechatpay-Serial"),
    };
    let app_maps = query_app_maps_by_pay_type(RdPayType::Wechat as i32);
    let message = format!("{}\n{}\n{}\n", header.time_stamp, header.nonce, body_str);
    let mut plaintext = String::new();
    for app_map in &app_maps {
        let client = build_client(app_map);
        if let Err(e) = futures::executor::block_on(client.auto_load_cert()) {
            error!(
                "load wechat cert failed: {}, app_id: {}",
                e, app_map.app_id
            );
            continue;
        }
        let verified = futures::executor::block_on(client.verify(
            &header.serial,
            &message,
            &header.signature,
        ));
        if !verified {
            warn!(
                "wechat notify signature verify failed, serial: {}, app_id: {}",
                header.serial, app_map.app_id
            );
            continue;
        }
        let envelope: WechatNotifyEnvelope = match serde_json::from_str(&body_str) {
            Ok(e) => e,
            Err(e) => {
                error!("parse wechat notify envelope failed: {}", e);
                return HttpResponse::Ok().json(WechatNotifyResponse::ok());
            }
        };
        if envelope.event_type != "TRANSACTION.SUCCESS" {
            warn!("wechat notify event type ignored: {}", envelope.event_type);
            return HttpResponse::Ok().json(WechatNotifyResponse::ok());
        }
        let crypto = WechatCryptoV3::new(&app_map.app_secret.clone().unwrap_or_default());
        match crypto.decrypt_data_v3(&envelope.resource) {
            Ok(bytes) => {
                plaintext = String::from_utf8_lossy(&bytes).to_string();
                break;
            }
            Err(e) => {
                error!("decrypt wechat notify failed: {}", e);
                return HttpResponse::Ok().json(WechatNotifyResponse::ok());
            }
        }
    }
    if plaintext.is_empty() {
        error!("wechat notify verify/decrypt failed, serial: {}", header.serial);
        return HttpResponse::Ok().json(WechatNotifyResponse::ok());
    }
    process_wechat_callback(&plaintext);
    return HttpResponse::Ok().json(WechatNotifyResponse::ok());
}

fn process_wechat_callback(plaintext: &str) {
    let transaction: WechatTransaction = match serde_json::from_str(plaintext) {
        Ok(t) => t,
        Err(e) => {
            error!("parse wechat transaction failed: {}, body: {}", e, plaintext);
            return;
        }
    };
    if transaction.trade_state != "SUCCESS" {
        warn!("wechat trade state not success: {}", transaction.trade_state);
        return;
    }
    let db_order = query_order_by_out_trans_no(&transaction.out_trade_no);
    let order_amount_fen =
        (db_order.total_price.clone().to_f64().unwrap_or_default() * 100.0).round() as i64;
    let pay_amount_fen = transaction.amount.as_ref().map(|a| a.total).unwrap_or(0);
    if order_amount_fen != pay_amount_fen {
        warn!(
            "wechat pay amount mismatch, order:{}, expect:{}, actual:{}",
            transaction.out_trade_no, order_amount_fen, pay_amount_fen
        );
        return;
    }
    if db_order.third_app_id != transaction.appid {
        warn!(
            "wechat appid mismatch, order:{}, expect:{}, actual:{}",
            transaction.out_trade_no, db_order.third_app_id, transaction.appid
        );
        return;
    }
    let payment_new = PaymentAdd {
        payment_id: transaction.transaction_id.clone(),
        order_id: transaction.out_trade_no.clone(),
        amount: BigDecimal::from(pay_amount_fen) / BigDecimal::from(100),
        status: RdPayStatus::Success as i32,
    };
    let mut connection = get_conn();
    let result: Result<Option<&str>, diesel::result::Error> = connection.transaction(|conn| {
        save_payment(&payment_new, conn);
        product_pay_success(&transaction.out_trade_no, conn);
        Ok(None)
    });
    if let Err(e) = result {
        error!("handle wechat pay callback failed, {}", e);
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/infra/wechat/notification").service(wechat_pay_notify);
    conf.service(scope);
}

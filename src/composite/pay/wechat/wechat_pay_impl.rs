use std::env;

use crate::{
    common::db::database::get_conn,
    model::{
        diesel::{
            custom::order::{order_add::OrderAdd, order_item_add::OrderItemAdd},
            dolphin::custom_dolphin_models::{AppMap, IapProduct},
        },
        pay::alipay_order_biz_content::AlipayOrderBizContent,
        resp::pay::alipay::order_resp::OrderResp,
    },
    service::{
        app::app_map_service::query_app_map_by_app_id, order::order_service::create_new_order,
    },
};
use bigdecimal::ToPrimitive;
use diesel::Connection;
use labrador::redis_store::RedisStorage;
use labrador::{TradeType, WechatPayClient, WechatPayRequestV3};
use log::error;
use rust_wheel::model::{enums::rd_pay_type::RdPayType, user::login_user_info::LoginUserInfo};
use rustflake::Snowflake;

pub fn do_wechat_pay(
    biz_content: &AlipayOrderBizContent,
    amap: &AppMap,
    iap: &IapProduct,
) -> Option<OrderResp> {
    // Build Wechat V3 unified order request from the existing biz content
    let notify_url = amap.notify_url.clone();
    let description = biz_content.subject.clone();
    // amount total is in cents for WeChat; biz_content.totalAmount is a float of yuan. Convert to fen (integer)
    let total_amount_fen = (biz_content.totalAmount * 100.0).round() as i64;

    let payer = None; // for app/payments payer may be omitted

    let req = WechatPayRequestV3 {
        appid: amap.third_app_id.clone().into(),
        mch_id: "".to_string(),
        notify_url: notify_url.unwrap_or_default(),
        description: description,
        out_trade_no: biz_content.outTradeNo.clone(),
        time_expire: "".to_string(),
        attach: None,
        amount: labrador::Amount {
            total: total_amount_fen,
            currency: String::from("CNY").into(),
            payer_total: None,
            payer_currency: None,
        },
        payer: payer,
        detail: None,
        scene_info: None,
        settle_info: None,
    };
    let secret = env::var("WECHAT_API_V3_KEY").unwrap_or_default();
    // create client and call unified_order_v3
    let binding = WechatPayClient::<RedisStorage>::new(&amap.third_app_id, &secret);
    let client = binding.wxpay();
    match futures::executor::block_on(client.create_order_v3(TradeType::App, req)) {
        Ok(pay_info) => {
            // pay_info should be a JSON Value containing payment info; stringify as formText
            let form_text = serde_json::to_string(&pay_info).unwrap_or_default();
            let order_resp = OrderResp {
                formText: form_text,
                orderId: biz_content.outTradeNo.to_string(),
                price: iap.price.to_string(),
            };
            Some(order_resp)
        }
        Err(err) => {
            error!(
                "do wechat pay error: {}, amap: {}",
                err,
                serde_json::to_string(amap).unwrap_or_default()
            );
            None
        }
    }
}

pub fn prepare_pay(login_user_info: &LoginUserInfo, iap: &IapProduct) -> OrderResp {
    let app_map = query_app_map_by_app_id(&login_user_info.appId, RdPayType::Alipay as i32);
    let mut snowflake = Snowflake::default();
    let snow_order_id = snowflake.generate().to_string();
    let biz_content = AlipayOrderBizContent {
        outTradeNo: snow_order_id.clone(),
        productCode: "FAST_INSTANT_TRADE_PAY".to_owned(),
        totalAmount: iap.price.to_f64().unwrap(),
        subject: iap.product_title.to_string(),
        qrPayMode: app_map.qr_pay_model.to_string(),
    };
    let env_seller_id: String = env::var("SELLER_ID").expect("seller id missing");
    let order_add = OrderAdd {
        user_id: login_user_info.userId,
        total_price: iap.price.clone(),
        third_app_id: app_map.third_app_id.clone(),
        app_id: app_map.app_id.clone(),
        pay_channel: RdPayType::Alipay as i32,
        qr_pay_model: app_map.qr_pay_model,
        subject: iap.product_title.clone(),
        product_code: "FAST_INSTANT_TRADE_PAY".to_owned(),
        order_id: snow_order_id.clone(),
        seller_id: env_seller_id,
    };
    let order_item = OrderItemAdd {
        order_id: snow_order_id.clone(),
        iap_product_id: iap.id,
        price: iap.price.clone(),
    };
    let mut connection = get_conn();
    let result: Result<Option<OrderResp>, diesel::result::Error> = connection.transaction(|conn| {
        let local_app_map = app_map.clone();
        let pay_result: Option<OrderResp> = do_wechat_pay(&biz_content, &local_app_map, iap);
        if pay_result.is_none() {
            error!("wechat pay result is null: {:?}", pay_result);
            return Ok(None);
        }
        let new_order = create_new_order(&order_add, conn, &order_item);
        if new_order.is_none() {
            error!("create wechat null order: {:?}", new_order);
            return Ok(None);
        }
        return Ok(pay_result);
    });
    if let Err(e) = result.as_ref() {
        error!("create wechat order failed, {}", e)
    }
    return result.unwrap().unwrap_or_default();
}

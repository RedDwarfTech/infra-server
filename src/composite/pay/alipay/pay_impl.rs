use crate::diesel::Connection;
use crate::service::order::order_service::create_new_order;
use bigdecimal::ToPrimitive;
use labrador::AlipayTradeWapPayModel;
use labrador::{
    redis_store::RedisStorage, AlipayBaseResponse, AlipayClient, AlipayTradeWapPayRequest
};
use log::{error, warn};
use rust_wheel::model::{enums::rd_pay_type::RdPayType, user::login_user_info::LoginUserInfo};
use rustflake::Snowflake;
use std::collections::BTreeMap;
use std::env;

use crate::model::diesel::custom::order::order_item_add::OrderItemAdd;
use crate::{
    common::db::database::get_conn,
    model::{
        diesel::{
            custom::order::order_add::OrderAdd,
            dolphin::custom_dolphin_models::{AppMap, IapProduct},
        },
        pay::alipay_order_biz_content::AlipayOrderBizContent,
    },
    service::app::app_map_service::query_app_map_by_app_id,
};

pub fn do_alipay(
    biz_content: &AlipayOrderBizContent,
    amap: &AppMap,
    iap: &IapProduct,
) -> Option<AlipayBaseResponse> {
    let pay_model = AlipayTradeWapPayModel {
        out_trade_no: biz_content.outTradeNo.clone(),
        total_amount: biz_content.totalAmount,
        subject: iap.product_title.clone(),
        body: Some("()".to_owned()),
        product_code: "()".to_owned(),
        auth_token: Some("()".to_owned()),
        quit_url: Some("()".to_owned()),
        goods_detail: None,
        extend_params: None,
        time_expire: Some("()".to_owned()),
        business_params: Some("()".to_owned()),
        passback_params: Some("()".to_owned()),
        merchant_order_no: Some(biz_content.outTradeNo.clone()),
    };
    let return_url = format!(
        "{}{}{}{}{}",
        amap.payed_redirect_url.clone().unwrap(),
        "?orderId=",
        biz_content.outTradeNo,
        "&payAmount=",
        biz_content.totalAmount
    );
    let mut udf_params = BTreeMap::new();
    udf_params.insert("key1".to_owned(), "value1".to_owned());
    let param = AlipayTradeWapPayRequest {
        api_version: "1.0".to_owned(),
        notify_url: amap.notify_url.clone(),
        return_url: Some(return_url),
        biz_content: Some(serde_json::to_string(biz_content).unwrap()),
        terminal_type: Some("".to_owned()),
        terminal_info: Some("".to_owned()),
        prod_code: Some("".to_owned()),
        biz_model: Some(pay_model),
        need_encrypt: false,
        udf_params: udf_params,
    };

    let client = AlipayClient::<RedisStorage>::new(&amap.third_app_id, false)
        .set_private_key(&amap.app_private_key).unwrap()
        .set_alipay_public_key(&amap.app_public_key)
        .set_sign_type("RSA2")
        .set_format("json")
        .set_charset("UTF-8");
    match client.wap_pay("POST".into(), param) {
        Ok(res) => {
            let r: AlipayBaseResponse = res;
            warn!("do alipay result: {}", serde_json::to_string(&r).unwrap());
            return Some(r);
        }
        Err(err) => {
            error!("do alipay error: {}, amap: {}", err, serde_json::to_string(amap).unwrap_or_default());
            None
        }
    }
}

pub fn prepare_pay(login_user_info: &LoginUserInfo, iap: &IapProduct) {
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
    let result: Result<Option<AlipayBaseResponse>, diesel::result::Error> =
        connection.transaction(|conn| {
            let local_app_map = app_map.clone();
            let pay_result: Option<AlipayBaseResponse> =
                do_alipay(&biz_content, &local_app_map, iap);
            if pay_result.is_some() {
                create_new_order(&order_add, conn, &order_item);
                return Ok(Some(pay_result.unwrap()));
            }
            Ok(None)
        });
    if let Err(e) = result {
        error!("create order failed, {}", e)
    }
}

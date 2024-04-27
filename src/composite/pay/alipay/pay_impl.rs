use labrador::{
    redis_store::RedisStorage, AlipayBaseResponse, AlipayClient, AlipayTradeWapPayRequest,
};
use log::{error, warn};
use rust_wheel::model::{enums::rd_pay_type::RdPayType, user::login_user_info::LoginUserInfo};
use rustflake::Snowflake;

use crate::{
    model::{diesel::dolphin::custom_dolphin_models::{AppMap, IapProduct}, pay::alipay_order_biz_content::AlipayOrderBizContent},
    service::app::app_map_service::query_app_map_by_app_id,
};

pub async fn do_alipay(iap: &IapProduct, amap: &AppMap) {
    let param = AlipayTradeWapPayRequest::default();
    let client = AlipayClient::<RedisStorage>::new(&amap.app_private_key, false);
    match client.wap_pay("POST".into(), param) {
        Ok(res) => {
            let r: AlipayBaseResponse = res;
            warn!("do alipay result: {}", serde_json::to_string(&r).unwrap());
        }
        Err(err) => {
            error!("do alipay error: {}", err);
        }
    }
}

pub fn prepare_pay(login_user_info: &LoginUserInfo, iap: &IapProduct) {
    let app_map = query_app_map_by_app_id(&login_user_info.appId, RdPayType::Alipay as i32);
    let mut snowflake = Snowflake::default();
    let biz_content = AlipayOrderBizContent{
        outTradeNo: snowflake.generate().to_string(),
        productCode: "FAST_INSTANT_TRADE_PAY".to_owned(),
        totalAmount: iap.price.to_string(),
        subject: iap.product_title.to_string(),
        qrPayMode: app_map.qr_pay_model.to_string(),
    };


    do_alipay(iap, &app_map);
}

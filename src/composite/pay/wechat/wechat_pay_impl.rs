use crate::{
    model::diesel::dolphin::custom_dolphin_models::IapProduct,
    service::app::app_map_service::query_app_map_by_app_id,
};
use paypal_rs::{
    api::orders::CreateOrder,
    data::{
        common::Currency,
        orders::{Amount, Intent, OrderPayloadBuilder, PurchaseUnit},
    },
    Client, PaypalEnv,
};
use rust_wheel::model::{enums::rd_pay_type::RdPayType, user::login_user_info::LoginUserInfo};

pub async fn do_wechat_pay(login_user_info: &LoginUserInfo, _iap: &IapProduct) {
    let app_map = query_app_map_by_app_id(&login_user_info.appId, RdPayType::Paypal as i32);
    let paypal_client_id = app_map.app_id;
    let paypal_secret = app_map.app_secret.unwrap_or_default();
    let mut client = Client::new(paypal_client_id, paypal_secret, PaypalEnv::Sandbox);
    client.get_access_token().await.unwrap();
    let order = OrderPayloadBuilder::default()
        .intent(Intent::Authorize)
        .purchase_units(vec![PurchaseUnit::new(Amount::new(Currency::EUR, "10.0"))])
        .build()
        .unwrap();
    let create_order = CreateOrder::new(order);
    let _order_created = client.execute(&create_order).await.unwrap();
}

use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use crate::model::diesel::dolphin::custom_dolphin_models::IapProduct;

#[derive(Deserialize, Serialize, Default, Clone)]
#[allow(non_snake_case)]
pub struct GoodsResp {
    pub id: i64,
    pub productId: i32,
    pub productType: i32,
    pub onlineStatus: i32,
    pub createdTime: i64,
    pub updatedTime: i64,
    pub productTitle: String,
    pub description: String,
    pub price: BigDecimal,
    pub rawPrice: BigDecimal,
    pub currencyCode: Option<String>,
    pub appId: String,
    pub sort: i32,
    pub deleted: i32,
    pub amount: i32,
    pub period: i32,
}

impl From<&IapProduct> for GoodsResp {
    fn from(iap: &IapProduct) -> Self {
        Self {
            id: iap.id,
            productId: iap.product_id,
            productType: iap.product_type,
            onlineStatus: iap.online_status,
            createdTime: iap.created_time,
            updatedTime: iap.updated_time,
            productTitle: iap.product_title.clone(),
            description: iap.description.clone(),
            price: iap.price.clone(),
            rawPrice: iap.raw_price.clone(),
            currencyCode: iap.currency_code.clone(),
            appId: iap.app_id.clone(),
            sort: iap.sort,
            deleted: iap.deleted,
            amount: iap.amount,
            period: iap.period,
        }
    }
}

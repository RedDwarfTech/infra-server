use serde::Serialize;
use validator::Validate;

#[derive(serde::Deserialize, Serialize, Validate)]
#[allow(non_snake_case)]
pub struct AlipayOrderBizContent {
    pub outTradeNo: String,
    pub productCode: String,
    pub totalAmount: f64,
    pub subject: String,
    pub qrPayMode: String,
}
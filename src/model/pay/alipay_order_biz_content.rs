use validator::Validate;

#[derive(serde::Deserialize, Validate)]
#[allow(non_snake_case)]
pub struct AlipayOrderBizContent {
    pub outTradeNo: String,
    pub productCode: String,
    pub totalAmount: String,
    pub subject: String,
    pub qrPayMode: String,
}
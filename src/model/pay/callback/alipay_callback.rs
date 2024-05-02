use bigdecimal::BigDecimal;
use serde::Serialize;
use validator::Validate;

/**
 * https://opendocs.alipay.com/open/270/105902?pathHash=d5cd617e
 */
#[derive(serde::Deserialize, Serialize, Validate, Debug)]
pub struct AlipayCallback {
    pub out_trade_no: String,
    pub trade_no: String,
    pub total_amount: BigDecimal,
}
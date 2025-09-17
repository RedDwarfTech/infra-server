use crate::composite::pay::alipay::alipay_callback_handler::handle_pay_callback;
use actix_web::{post, web, HttpResponse, Responder};
use labrador::{serde_urlencoded::to_string, AlipayNotifyResponse};

/// Recieve notifycation
///
/// Recieve notifycation
/// The alipay will send a post request
/// but the params will put into request params
/// the request will look like this:
/// https: //商家网站通知地址?voucher_detail_list=[{"amount":"0.20","merchantContribute":"0.00","name":"5折券","otherContribute":"0.20","type":"ALIPAY_DISCOUNT_VOUCHER","voucherId":"2016101200073002586200003BQ4"}]&fund_bill_list=[{"amount":"0.80","fundChannel":"ALIPAYACCOUNT"},{"amount":"0.20","fundChannel":"MDISCOUNT"}]&subject=PC网站支付交易&trade_no=2016101221001004580200203978&gmt_create=2016-10-12 21:36:12&notify_type=trade_status_sync&total_amount=1.00&out_trade_no=mobile_rdm862016-10-12213600&invoice_amount=0.80&seller_id=2088201909970555&notify_time=2016-10-12 21:41:23&trade_status=TRADE_SUCCESS&gmt_payment=2016-10-12 21:37:19&receipt_amount=0.80&passback_params=passback_params123&buyer_id=2088102114562585&app_id=2016092101248425&notify_id=7676a2e1e4e737cff30015c4b7b55e3kh6& sign_type=RSA2&buyer_pay_amount=0.80&sign=***&point_amount=0.00
/// so we still need to parse the params from url
/// not from request body
/// for a long time try and try, the request params was submitted by form
/// so parse the data from form
/// https://opendocs.alipay.com/open/270/105902?pathHash=d5cd617e
/// https://github.com/seanmonstar/httparse/issues/146
/// https://opensupport.alipay.com/support/technical-investigation
#[utoipa::path(
    context_path = "/infra/alipay/pay",
    path = "/",
    responses(
        (status = 200, description = "Recieve notification")
    )
)]
#[post("/v1/alipaySeverNotification")]
pub async fn alipay_server_notify(form: web::Form<AlipayNotifyResponse>) -> impl Responder {
    let query_string = to_string(&form.0).unwrap();
    handle_pay_callback(&query_string);
    return HttpResponse::Ok().body("failed");
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/infra/alipay/notification").service(alipay_server_notify);
    conf.service(scope);
}

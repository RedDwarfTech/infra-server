use actix_web::{post, web, Responder};
use alibaba_cloud_sdk_rust::services::dysmsapi;
use gostd::strings;
use rust_wheel::common::wrapper::actix_http_resp::box_actix_rest_response;

use crate::model::req::notify::sms::sms_req::SmsReq;

const AliyunSmsServerRegion: &str = "cn-hangzhou";
const AliyunSmsAccessKeyID: &str = "LTAI4FwqPxiAxxxxxx";
const AliyunSmsAccessKeySecret: &str = "xxxxx0FJqHTTLwDUuhxxxxx";
const AliyunSmsSignName: &str = "阿里云"; // 短信署名

/// Send sms message
///
/// send sms message
#[utoipa::path(
    context_path = "/infra-inner/sms/send",
    path = "/",
    responses(
        (status = 200, description = "get order status")
    )
)]
#[post("/send")]
pub async fn send(_params: web::Query<SmsReq>) -> impl Responder {
    // the sms quota of each app check
    match send_sms(&_params.0) {
        Ok(response) => box_actix_rest_response(response),
        Err(_err) => box_actix_rest_response("err"),
    }
}

fn send_sms(sms_req: &SmsReq) -> Result<(), std::io::Error> {
    let mut client = dysmsapi::Client::NewClientWithAccessKey(
        AliyunSmsServerRegion,
        AliyunSmsAccessKeyID,
        AliyunSmsAccessKeySecret,
    )?;
    let mut request = dysmsapi::CreateSendSmsRequest();
    request.PhoneNumbers = strings::Replace(sms_req.phone.clone(), "+86", "", -1);
    request.SignName = AliyunSmsSignName.to_owned();
    request.TemplateCode = sms_req.tpl_code.to_owned();
    let response = client.SendSms(&mut request)?;
    println!("{:?}", &response);

    Ok(())
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/infra-inner/sms").service(send);
    conf.service(scope);
}

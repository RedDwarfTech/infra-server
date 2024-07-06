use alibaba_cloud_sdk_rust::services::dysmsapi;
use gostd::strings;

use crate::model::req::notify::sms::sms_req::SmsReq;

const AliyunSmsServerRegion: &str = "cn-hangzhou";
const AliyunSmsAccessKeyID: &str = "LTAI4FwqPxiAxxxxxx";
const AliyunSmsAccessKeySecret: &str = "xxxxx0FJqHTTLwDUuhxxxxx";
const AliyunSmsSignName: &str = "阿里云"; // 短信署名

pub fn send_sms(sms_req: &SmsReq) -> Result<(), std::io::Error> {
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

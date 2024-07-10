use crate::{
    model::{diesel::dolphin::custom_dolphin_models::SmsConfig, req::notify::sms::sms_req::SmsReq},
    service::notify::sms_config_service::get_app_sms_config,
};
use alibaba_cloud_sdk_rust::services::dysmsapi;
use gostd::strings;

pub fn send_sms(sms_req: &SmsReq) -> Result<(), std::io::Error> {
    let sms_conf: SmsConfig = get_app_sms_config(&sms_req.app_id);
    let mut client = dysmsapi::Client::NewClientWithAccessKey(
        sms_conf.server_region.unwrap().as_str(),
        &sms_conf.access_key_id,
        &sms_conf.access_key_secret,
    )?;
    let mut request = dysmsapi::CreateSendSmsRequest();
    request.PhoneNumbers = strings::Replace(sms_req.phone.clone(), "+86", "", -1);
    request.SignName = sms_conf.sign_name.to_owned();
    request.TemplateCode = sms_req.tpl_code.to_owned();
    let response = client.SendSms(&mut request)?;
    println!("{:?}", &response);

    Ok(())
}

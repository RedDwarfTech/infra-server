use std::collections::HashMap;

use crate::{
    model::{diesel::dolphin::custom_dolphin_models::SmsConfig, req::notify::sms::sms_req::SmsReq},
    service::notify::sms_config_service::get_app_sms_config,
};
use alibaba_cloud_sdk_rust::services::dysmsapi::{self, SendSmsRequest, SendSmsResponse};
use gostd::strings;
use log::error;
use rand::Rng;

pub fn send_sms(sms_req: &SmsReq) -> Option<SendSmsResponse> {
    let sms_conf: SmsConfig = get_app_sms_config(&sms_req.app_id);
    let client = dysmsapi::Client::NewClientWithAccessKey(
        sms_conf.server_region.unwrap().as_str(),
        &sms_conf.access_key_id,
        &sms_conf.access_key_secret,
    );
    if let Err(err) = client {
        error!("initial client failed,{}", err);
        return None;
    }
    let mut request: SendSmsRequest = dysmsapi::CreateSendSmsRequest();
    request.PhoneNumbers = strings::Replace(sms_req.phone.clone(), "+86", "", -1);
    request.SignName = sms_conf.sign_name.to_owned();
    request.TemplateCode = sms_req.tpl_code.to_owned();
    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.gen_range(100000..=999999);
    let mut params = HashMap::new();
    params.insert("code", random_number);
    request.TemplateParam = serde_json::to_string(&params).unwrap();
    let response = client.unwrap().SendSms(&mut request);
    match response {
        Ok(response) => {
            let  resp = serde_json::from_slice(&response.httpContentBytes);
            return Some(resp.unwrap_or_default());
        },
        Err(err) => {
            error!("send sms message facing issue: {:?}", &err);
            return None;
        }
    }
}

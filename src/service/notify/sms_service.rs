use std::{collections::HashMap, env};

use crate::{
    model::{diesel::dolphin::custom_dolphin_models::SmsConfig, req::notify::sms::sms_req::SmsReq},
    service::notify::sms_config_service::get_app_sms_config,
};
use alibaba_cloud_sdk_rust::services::dysmsapi::{self, SendSmsRequest, SendSmsResponse};
use gostd::strings;
use log::error;

pub fn send_sms(sms_req: &SmsReq, params: HashMap<&str,String>) -> Option<SendSmsResponse> {
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
    request.TemplateParam = serde_json::to_string(&params).unwrap();
    let ignore_url: String = env::var("SMS_TEST_PHONE").expect("ignore url config missing");
    let parts: Vec<String> = ignore_url.split(',').map(|s| s.to_string()).collect();
    if parts.contains(&sms_req.phone.clone()) {
        let resp: SendSmsResponse = SendSmsResponse {
            RequestId: "1".to_string(),
            BizId: "1".to_string(),
            Code: "12".to_owned(),
            Message: "ok".to_owned(),
        };
        return Some(resp);
    }
    let response = client.unwrap().SendSms(&mut request);
    match response {
        Ok(response) => {
            let resp = serde_json::from_slice(&response.httpContentBytes);
            return Some(resp.unwrap_or_default());
        }
        Err(err) => {
            error!("send sms message facing issue: {:?}", &err);
            return None;
        }
    }
}

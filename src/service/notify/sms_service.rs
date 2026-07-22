use std::{collections::HashMap, env};

use crate::{
    model::{diesel::dolphin::custom_dolphin_models::SmsConfig, req::notify::sms::sms_req::SmsReq},
    service::notify::sms_config_service::get_app_sms_config,
};
use alibaba_cloud_sdk_rust::services::dysmsapi::{self, SendSmsRequest, SendSmsResponse};
use gostd::strings;
use log::{error, info, warn};

pub fn send_sms(sms_req: &SmsReq, params: HashMap<&str, String>) -> Option<SendSmsResponse> {
    let sms_conf: SmsConfig = get_app_sms_config(&sms_req.app_id);
    let region = match &sms_conf.server_region {
        Some(r) if !r.is_empty() => r.as_str(),
        _ => {
            error!(
                "send_sms missing server_region, phone:{}, app_id:{}, tpl_code:{}",
                sms_req.phone, sms_req.app_id, sms_req.tpl_code
            );
            return None;
        }
    };
    let client = dysmsapi::Client::NewClientWithAccessKey(
        region,
        &sms_conf.access_key_id,
        &sms_conf.access_key_secret,
    );
    if let Err(err) = client {
        error!(
            "send_sms init aliyun client failed, phone:{}, app_id:{}, region:{}, err:{}",
            sms_req.phone, sms_req.app_id, region, err
        );
        return None;
    }
    let mut request: SendSmsRequest = dysmsapi::CreateSendSmsRequest();
    request.PhoneNumbers = strings::Replace(sms_req.phone.clone(), "+86", "", -1);
    request.SignName = sms_conf.sign_name.to_owned();
    request.TemplateCode = sms_req.tpl_code.to_owned();
    request.TemplateParam = serde_json::to_string(&params).unwrap();
    info!(
        "send_sms request prepared, phone:{}, app_id:{}, region:{}, sign_name:{}, tpl_code:{}, template_param_keys:{:?}",
        sms_req.phone,
        sms_req.app_id,
        region,
        request.SignName,
        request.TemplateCode,
        params.keys().collect::<Vec<_>>()
    );
    let ignore_url: String = env::var("SMS_TEST_PHONE").expect("ignore url config missing");
    let parts: Vec<String> = ignore_url.split(',').map(|s| s.to_string()).collect();
    if parts.contains(&sms_req.phone.clone()) {
        info!(
            "send_sms bypassed by SMS_TEST_PHONE, phone:{}, app_id:{}, tpl_code:{}",
            sms_req.phone, sms_req.app_id, sms_req.tpl_code
        );
        let resp: SendSmsResponse = SendSmsResponse {
            RequestId: "1".to_string(),
            BizId: "1".to_string(),
            Code: "OK".to_owned(),
            Message: "ok".to_owned(),
        };
        return Some(resp);
    }
    let response = client.unwrap().SendSms(&mut request);
    match response {
        Ok(response) => {
            let http_status = response.httpStatus;
            let body = &response.httpContentString;
            let body_len = response.httpContentBytes.len();
            if body.trim().is_empty() {
                error!(
                    "send_sms empty http body, phone:{}, app_id:{}, tpl_code:{}, http_status:{}, body_len:{}",
                    sms_req.phone, sms_req.app_id, sms_req.tpl_code, http_status, body_len
                );
                return None;
            }
            match serde_json::from_slice::<SendSmsResponse>(&response.httpContentBytes) {
                Ok(resp) => {
                    if resp.Code == "OK" {
                        info!(
                            "send_sms success, phone:{}, app_id:{}, tpl_code:{}, http_status:{}, request_id:{}, biz_id:{}, message:{}",
                            sms_req.phone,
                            sms_req.app_id,
                            sms_req.tpl_code,
                            http_status,
                            resp.RequestId,
                            resp.BizId,
                            resp.Message
                        );
                    } else {
                        warn!(
                            "send_sms aliyun rejected, phone:{}, app_id:{}, tpl_code:{}, http_status:{}, aliyun_code:{}, aliyun_message:{}, request_id:{}, biz_id:{}, raw_body:{}",
                            sms_req.phone,
                            sms_req.app_id,
                            sms_req.tpl_code,
                            http_status,
                            resp.Code,
                            resp.Message,
                            resp.RequestId,
                            resp.BizId,
                            body
                        );
                    }
                    Some(resp)
                }
                Err(err) => {
                    error!(
                        "send_sms parse response failed, phone:{}, app_id:{}, tpl_code:{}, http_status:{}, body_len:{}, err:{}, raw_body:{}",
                        sms_req.phone,
                        sms_req.app_id,
                        sms_req.tpl_code,
                        http_status,
                        body_len,
                        err,
                        body
                    );
                    None
                }
            }
        }
        Err(err) => {
            error!(
                "send_sms sdk call failed, phone:{}, app_id:{}, tpl_code:{}, sign_name:{}, region:{}, err:{:?}",
                sms_req.phone,
                sms_req.app_id,
                sms_req.tpl_code,
                request.SignName,
                region,
                err
            );
            None
        }
    }
}

use std::collections::HashMap;

use crate::{
    common::db::database::get_conn,
    composite::user::user_product_sub_handler::product_pay_success,
    model::diesel::{custom::pay::payment_add::PaymentAdd, dolphin::custom_dolphin_models::AppMap},
    service::{
        app::app_map_service::query_app_map_by_third_app_id,
        pay::sys::payment_service::save_payment,
    },
};
use actix_web::cookie::time::util::is_leap_year;
use base64::decode;
use diesel::Connection;
use form_urlencoded::Parse;
use log::{error, warn};
use rust_wheel::alipay::api::internal::util::sign::{format_pem_public_key, load_public_key, Signer};
use rust_wheel::{
    alipay::api::internal::util::{
        alipay_signature::{get_sign_check_content_v1, rsa_check_v1},
        sign::builder,
    },
    model::enums::{rd_pay_status::RdPayStatus, rd_pay_type::RdPayType},
};
use std::error::Error;

///
/// https://opendocs.alipay.com/open/270/105902?pathHash=d5cd617e
///
pub fn handle_pay_callback(query_string: &String) {
    let mut params: HashMap<String, String> = parse_query(query_string);
    // verify signature
    // 第一步：在通知返回参数列表中，除去 sign、sign_type 两个参数外，凡是通知返回回来的参数皆是待验签的参数。
    // 第二步：将剩下参数进行 url_decode，然后进行字典排序，组成字符串，得到待签名字符串
    // 第三步：将签名参数（sign）使用 base64 解码为字节码串。
    // 第四步：使用 RSA 的验签方法，通过签名字符串、签名参数（经过 base64 解码）及支付宝公钥验证签名。
    // 第五步：需要严格按照如下描述校验通知数据的正确性：
    // 1. 商家需要验证该通知数据中的 out_trade_no 是否为商家系统中创建的订单号。
    // 2. 判断 total_amount 是否确实为该订单的实际金额（即商家订单创建时的金额）。
    // 3. 校验通知中的 seller_id（或者 seller_email）是否为 out_trade_no 这笔单据的对应的操作方（有的时候，一个商家可能有多个 seller_id/seller_email）。
    // 4. 验证 app_id 是否为该商家本身。
    let cb_app_id = params.get("app_id").unwrap();
    let cb_sign = params.get("sign").unwrap();
    let appmap = query_app_map_by_third_app_id(cb_app_id, RdPayType::Alipay as i32);
    let verify_result = verify_callback(&appmap, &mut params.clone(), &cb_sign);
    match verify_result {
        Ok(pass) => {
            if pass {
                warn!("verify pass, data: {},", pass);
            } else {
                warn!("verify not pass, callback sign:{},params:{:?}", urlencoding::decode(cb_sign).unwrap_or_default().into_owned(), params);
            }
        }
        Err(e) => {
            error!("verify facing error, {}, callback sign: {}", e, cb_sign);
        }
    }
    _legacy_verify(&appmap,&mut params);
}

fn verify_callback(
    appmap: &AppMap,
    params: &mut HashMap<String, String>,
    signature: &String,
) -> Result<bool, std::io::Error> {
    let mut sign = builder().sign_type_rsa2().build();
    sign.set_private_key(&appmap.app_private_key_pkcs1)?;
    sign.set_public_key(&appmap.alipay_public_key)?;
    let sorted_source = get_sign_check_content_v1(params);
    let naked_sorted_source = sorted_source.unwrap_or_default();
    warn!("naked_sorted_source: {}", naked_sorted_source);
    let decoded_pairs = form_urlencoded::parse(&naked_sorted_source.as_bytes());
    let mut decoded_str = String::new();
    for (key, value) in decoded_pairs {
        decoded_str.push_str(&format!("{}={}&", key, value));
    }
    // 去除最后一个 "&"
    decoded_str.pop();
    warn!("pass to verify source: {}, sign: {}", decoded_str, signature);
    let is_passed: Result<bool, std::io::Error> = sign.verify(
        &decoded_str,
        &signature,
    );
    
    return is_passed;
}

fn _legacy_verify(appmap: &AppMap, params: &mut HashMap<String, String>) {
    // load the der format public key
    let der_public_key = format_pem_public_key(&appmap.alipay_public_key.clone());
    let verify_result = rsa_check_v1(params, der_public_key);
    match verify_result {
        Ok(_data) => {
            // process_callback(params);
            error!("legacy success")
        }
        Err(err) => {
            error!("legacy verify failed, params: {:?}, err:{:?}", params, err);
            return;
        }
    }
}

fn process_callback(params: &mut HashMap<String, String>) {
    let cb_order_id = params.get("out_trade_no").unwrap();
    let cb_payment_id = params.get("trade_no").unwrap();
    let total_amount = params.get("total_amount").unwrap();
    let payment_new = PaymentAdd {
        payment_id: cb_payment_id.to_string(),
        order_id: cb_order_id.to_string(),
        amount: total_amount
            .parse()
            .expect("Failed to parse BigDecimal in alipay callback"),
        status: RdPayStatus::Success as i32,
    };
    let mut connection = get_conn();
    let result: Result<Option<&str>, diesel::result::Error> = connection.transaction(|conn| {
        save_payment(&payment_new, conn);
        product_pay_success(&cb_order_id, conn);
        Ok(None)
    });
    if let Err(e) = result {
        error!("handle pay callback failed, {}", e);
    }
}

fn parse_query(query: &str) -> HashMap<String, String> {
    // https://stackoverflow.com/questions/43272935/how-do-i-decode-a-url-and-get-the-query-string-as-a-hashmap
    query
        .split('&')
        .filter_map(|s| {
            s.split_once('=')
                .and_then(|t| Some((t.0.to_owned(), t.1.to_owned())))
        })
        .collect()
}

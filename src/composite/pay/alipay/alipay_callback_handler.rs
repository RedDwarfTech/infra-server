use std::collections::HashMap;

use crate::service::order::order_service::query_order_by_out_trans_no;
use crate::{
    common::db::database::get_conn,
    composite::user::user_product_sub_handler::product_pay_success,
    model::diesel::{custom::pay::payment_add::PaymentAdd, dolphin::custom_dolphin_models::AppMap},
    service::{
        app::app_map_service::query_app_map_by_third_app_id,
        pay::sys::payment_service::save_payment,
    },
};
use diesel::Connection;
use log::{error, warn};
use rust_wheel::alipay::api::internal::util::alipay_signature::rd_rsa_check_v1;
use rust_wheel::alipay::api::internal::util::sign::Signer;
use rust_wheel::{
    alipay::api::internal::util::{alipay_signature::get_sign_check_content_v1, sign::builder},
    model::enums::{rd_pay_status::RdPayStatus, rd_pay_type::RdPayType},
};

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
                process_callback(&mut params);
            } else {
                error!(
                    "verify not pass, callback sign:{},params:{:?}",
                    urlencoding::decode(cb_sign)
                        .unwrap_or_default()
                        .into_owned(),
                    params
                );
            }
        }
        Err(e) => {
            error!("verify facing error, {}, callback sign: {}", e, cb_sign);
        }
    }
}

/** 验证回调，确保是由合法的调用方发起
 *
 * 关于回调的签名验证，也是花了不少时间，总结下来，需要注意以下几点：
 * 1、验证签名用支付宝的公钥，不是app的公钥
 * 2、公钥的格式注意是der格式，支付宝提供的公钥要适当的格式化，不能直接使用
 * 3、需要细心和耐心，积极联系支付宝的技术支持，中途遇到验证不通过，经过和技术支持的字符串
 * 对比才发现rust自动将浮点数0.00转换成了0.0，参考：https://stackoverflow.com/questions/78431631/how-to-keep-fixed-precision-when-recieved-f64-value-in-rust
 * 如果没有技术支持在后台能够看到支付宝发出的原始字符串，
 * 找出问题基本是不可能的，这个奇怪的问题花了相当长时间，技术支持的说辞是要保证参数的顺序，
 * 很明显在这个问题上技术支持缺乏专业度
 * 4、解码和编码的规则细微差异，比如解码时间时是否处理+符号，是否替换为空格，这些细节的问题很难察觉，
 * 就会导致签名一直验证失败，但是需要细心才能发现问题，同样需要感谢技术支持提供的标准响应内容
 * 更多信息参考：https://stackoverflow.com/questions/78430980/make-whitespace-to-breaks-date-and-time-in-url-using-rust
 *
 * # 参数
 *
 * * `appmap` - 所有三方app的映射配置，存储在数据库中
 * * `params` - 待验证签名的参数
 *
 * # 返回
 *
 * 返回是否成功，成功表示由合法的调用方发起，继续后续的处理流程
 */
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
    let decoded_pairs = form_urlencoded::parse(&naked_sorted_source.as_bytes());
    let mut decoded_str = String::new();
    for (key, value) in decoded_pairs {
        decoded_str.push_str(&format!("{}={}&", key, value));
    }
    // 去除最后一个 "&"
    decoded_str.pop();
    let base64_dec = urlencoding::decode(signature);
    let decoded_sign = base64_dec.unwrap_or_default().into_owned();
    let is_passed: Result<bool, std::io::Error> = sign.verify(&decoded_str, &decoded_sign);
    _legacy_verify(&appmap, &decoded_str, &decoded_sign);
    return is_passed;
}

fn _legacy_verify(appmap: &AppMap, decoded_str: &String, decoded_sign: &String) {
    let verify_result =
        rd_rsa_check_v1(decoded_str, decoded_sign, appmap.alipay_public_key.clone());
    match verify_result {
        Ok(_data) => {
            // process_callback(params);
            error!("legacy success")
        }
        Err(err) => {
            error!(
                "legacy verify failed, decoded_str: {:?}, err:{:?}",
                decoded_str, err
            );
            return;
        }
    }
}

fn process_callback(params: &mut HashMap<String, String>) {
    let succ = verify_invalid_callback(params);
    if !succ {
        warn!("illegal invoke from alipay, {:?}", params);
        return;
    }
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

///
/// https://opendocs.alipay.com/open/270/105902?pathHash=d5cd617e
///
fn verify_invalid_callback(params: &mut HashMap<String, String>) -> bool {
    let cb_order_id = params.get("out_trade_no").unwrap();
    let total_amount = params.get("total_amount").unwrap();
    let cb_seller_id = params.get("seller_id").unwrap();
    let cb_app_id = params.get("app_id").unwrap();
    // 商家需要验证该通知数据中的 out_trade_no 是否为商家系统中创建的订单号
    let db_order = query_order_by_out_trans_no(cb_order_id);
    // 判断 total_amount 是否确实为该订单的实际金额（即商家订单创建时的金额）
    if db_order.total_price.to_string() != total_amount.to_owned() {
        warn!(
            "the order price did not match, db:{}, total amount:{}",
            db_order.total_price,
            total_amount.to_owned()
        );
        return false;
    }
    // 校验通知中的 seller_id（或者 seller_email）是否为 out_trade_no
    // 这笔单据的对应的操作方（有的时候，一个商家可能有多个 seller_id/seller_email）
    if db_order.seller_id != cb_seller_id.to_owned() {
        warn!(
            "seller id did not match, db:{},cb:{}",
            db_order.seller_id, cb_seller_id
        );
        return false;
    }
    // 验证 app_id 是否为该商家本身
    if db_order.third_app_id != cb_app_id.to_owned() {
        warn!(
            "app id not match, db:{},cb:{}",
            db_order.third_app_id,
            cb_app_id.to_owned()
        );
        return false;
    }
    return true;
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

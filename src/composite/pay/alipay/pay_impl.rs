use labrador::{redis_store::RedisStorage, AlipayBaseResponse, AlipayClient, AlipayTradeWapPayRequest};
use log::{error, warn};

pub async fn do_alipay() {
    let param = AlipayTradeWapPayRequest::default();
    let client = AlipayClient::<RedisStorage>::new("appKey", false);
    match client.wap_pay("POST".into(), param) {
        Ok(res) => {
            let r:AlipayBaseResponse = res;
            warn!("do alipay result: {}", serde_json::to_string(&r).unwrap());
        }
        Err(err) => {
            error!("do alipay error: {}", err);
        }
    }
}

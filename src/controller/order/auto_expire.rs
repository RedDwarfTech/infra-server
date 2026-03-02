use chrono::Utc;
use log::{error, info, warn};
use std::time::Duration;
use uuid::Uuid;
use rust_wheel::model::enums::order::rd_order_status::RdOrderStatus;
use ::tokio::task;
use ::tokio::spawn;
use ::tokio::time;
use crate::common::db::database::get_conn;
use crate::model::diesel::dolphin::dolphin_schema::orders as orders_table;
use crate::model::diesel::dolphin::custom_dolphin_models::Order;
use crate::service::order::order_service::update_order_status;

const LOCK_KEY: &str = "infra:auto_expire:lock";
const LOCK_TTL_MS: usize = 20_000; // 20s lock TTL
const INTERVAL_SECS: u64 = 30; // each node checks every 30s but only lock holder runs expiration
const BATCH_SIZE: i64 = 1;
const EXPIRE_SECONDS: i64 = 1800; // 30 minutes

async fn acquire_lock(
    con: &mut redis::aio::Connection,
    key: &str,
    val: &str,
    ttl_ms: usize,
) -> redis::RedisResult<bool> {
    // SET key val NX PX ttl
    let mut cmd = redis::cmd("SET");
    cmd.arg(key).arg(val).arg("NX").arg("PX").arg(ttl_ms);
    let res: Option<String> = cmd.query_async(con).await?;
    Ok(res.is_some())
}

async fn release_lock(
    con: &mut redis::aio::Connection,
    key: &str,
    val: &str,
) -> redis::RedisResult<()> {
    // safe release via Lua: if redis.call('get',KEY)==ARGV[1] then return redis.call('del',KEY) else return 0 end
    let script = r#"if redis.call('get',KEYS[1]) == ARGV[1] then return redis.call('del',KEYS[1]) else return 0 end"#;
    let _: i32 = redis::Script::new(script).key(key).arg(val).invoke_async(con).await?;
    Ok(())
}

async fn do_expire_once() {
    // perform DB scan and expire in a blocking task to avoid blocking tokio reactor
    let now = Utc::now().timestamp();
    let expire_before = now - EXPIRE_SECONDS;
    let processed = task::spawn_blocking(move || {
        let mut conn = get_conn();
        use diesel::prelude::*;
        let predicate = orders_table::order_status
            .eq(RdOrderStatus::WaitingForPayment as i32)
            .and(orders_table::created_time.le(expire_before));
        let items: QueryResult<Vec<Order>> = orders_table::table
            .filter(predicate)
            .limit(BATCH_SIZE)
            .load::<Order>(&mut conn);
        match items {
            Ok(list) => {
                let mut cnt = 0;
                for o in list.iter() {
                    update_order_status(&o.id, RdOrderStatus::EXPIRED as i32, &mut conn);
                    cnt += 1;
                }
                Ok::<i32, String>(cnt)
            }
            Err(e) => Err(format!("query expire orders failed: {}", e)),
        }
    })
    .await;

    match processed {
        Ok(Ok(cnt)) => {
            if cnt > 0 {
                info!("auto_expire: expired {} orders", cnt);
            }
        }
        Ok(Err(e)) => error!("auto_expire db error: {}", e),
        Err(e) => error!("auto_expire spawn_blocking join error: {}", e),
    }
}

pub fn start_auto_expire_task() {
    // spawn background async task
    spawn(async move {
        // create redis client
        let redis_url = match std::env::var("REDIS_URL") {
            Ok(u) => u,
            Err(_) => {
                warn!("REDIS_URL not set, auto_expire disabled");
                return;
            }
        };
        let client = match redis::Client::open(redis_url.as_str()) {
            Ok(c) => c,
            Err(e) => {
                error!("create redis client failed: {}", e);
                return;
            }
        };
        let mut con = match client.get_tokio_connection().await {
            Ok(c) => c,
            Err(e) => {
                error!("get redis connection failed: {}", e);
                return;
            }
        };

        let mut interval = time::interval(Duration::from_secs(INTERVAL_SECS));
        loop {
            interval.tick().await;

            let uuid = Uuid::new_v4().to_string();
            let got = match acquire_lock(&mut con, LOCK_KEY, &uuid, LOCK_TTL_MS).await {
                Ok(v) => v,
                Err(e) => {
                    error!("auto_expire acquire lock err: {}", e);
                    false
                }
            };

            if !got {
                // not leader this round
                continue;
            }

            // we are leader, run expiration logic
            let _ = do_expire_once().await;

            // try release lock (best-effort)
            if let Err(e) = release_lock(&mut con, LOCK_KEY, &uuid).await {
                error!("auto_expire release lock err: {}", e);
            }
        }
    });
}

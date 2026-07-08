use crate::controller::utils::snowflake::SnowflakeIdWorker;
use actix_web::{get, web, Responder};
use log::error;
use rust_wheel::common::wrapper::actix_http_resp::{
    box_actix_rest_response, box_error_actix_rest_response,
};
use std::sync::OnceLock;

static SNOWFLAKE_WORKER: OnceLock<SnowflakeIdWorker> = OnceLock::new();

/// Derives Snowflake worker and datacenter IDs from the pod hostname.
///
/// Reads the `HOSTNAME` environment variable, which on Kubernetes StatefulSets
/// follows `{statefulset-name}-{index}`. The numeric suffix is clamped to 0–1023
/// and split into two 5-bit fields: lower bits become `worker_id`, upper bits
/// become `datacenter_id`.
fn resolve_worker_ids() -> (i64, i64) {
    let hostname = std::env::var("HOSTNAME").unwrap_or_default();
    let index = hostname
        .split('-')
        .last()
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(0)
        .min(1023);
    let worker_id = (index & 31) as i64;
    let datacenter_id = ((index >> 5) & 31) as i64;
    (worker_id, datacenter_id)
}

/// Returns the process-wide [`SnowflakeIdWorker`] singleton.
///
/// Initialized lazily on first use via [`OnceLock`]. A single worker must be
/// reused so that `last_timestamp` and `sequence` persist across requests.
fn snowflake_id_worker() -> &'static SnowflakeIdWorker {
    SNOWFLAKE_WORKER.get_or_init(|| {
        let (worker_id, datacenter_id) = resolve_worker_ids();
        SnowflakeIdWorker::new(worker_id, datacenter_id)
            .expect("failed to initialize SnowflakeIdWorker")
    })
}

/// Generates a globally unique Snowflake ID for internal callers.
///
/// Returns a 64-bit integer encoding a custom epoch timestamp, datacenter ID,
/// worker ID, and a per-millisecond sequence. IDs are monotonically increasing
/// within a single process under normal clock behaviour.
///
/// # Worker identity
///
/// On Kubernetes StatefulSets, the pod ordinal is derived from the `HOSTNAME`
/// environment variable (`{statefulset-name}-{index}`). The ordinal (clamped to
/// 0–1023) is split into two 5-bit fields: the lower bits become `worker_id`,
/// the upper bits become `datacenter_id`, supporting up to 32 × 32 = 1024
/// distinct generators per cluster layout.
///
/// # Process-wide generator
///
/// A single [`SnowflakeIdWorker`] is held in a [`OnceLock`] for the lifetime of
/// the process. Reusing one worker preserves `last_timestamp` and `sequence`
/// across HTTP requests; constructing a new worker on every call would reset
/// both counters and can emit duplicate IDs when multiple requests arrive in
/// the same millisecond.
///
/// # Clock rollback
///
/// Wall-clock backward steps are handled inside [`SnowflakeIdWorker::next_id`]:
/// small drift waits for the clock to catch up; larger drift within a bounded
/// window uses a logical timestamp so generation can continue without breaking
/// monotonicity. Generation fails only when drift exceeds the configured
/// maximum—see [`crate::controller::utils::snowflake`] for the full policy.
///
/// # Errors
///
/// Responds with an HTTP error payload when ID generation fails (for example,
/// clock drift beyond the maximum logical drift window).
#[utoipa::path(
    context_path = "/infra-inner/util/uniqid/gen",
    path = "/",
    responses(
        (status = 200, description = "Generates a globally unique Snowflake ID for internal callers")
    )
)]
#[get("/uniqid/gen")]
pub async fn id_gen() -> impl Responder {
    match snowflake_id_worker().next_id() {
        Ok(id) => box_actix_rest_response(id),
        Err(e) => {
            error!("gen uniq id failed, {}", e);
            box_error_actix_rest_response(0i64, "0030010099".to_string(), e.to_string())
        }
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/infra-inner/util").service(id_gen);
    conf.service(scope);
}

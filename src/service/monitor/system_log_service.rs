use log::error;
use reqwest::Client;
use serde_json::{json, Value};
use std::time::Duration;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::config::app::app_conf_reader::get_app_config;

use crate::common::db::database::get_conn;
use crate::diesel::RunQueryDsl;
use crate::model::diesel::custom::system_log::system_log_add::SystemLogAdd;
use crate::model::diesel::dolphin::custom_dolphin_models::SystemLog;

pub fn save_system_log(system_log: &SystemLogAdd) {
    use crate::model::diesel::dolphin::dolphin_schema::system_log as query_table;
    let result = diesel::insert_into(query_table::dsl::system_log)
        .values(system_log)
        .get_result::<SystemLog>(&mut get_conn());
    if let Err(e) = result {
        error!("insert system log failed: {:?}, system log: {:?}", e, system_log);
    }
}

pub fn save_system_log_with_now(system_log: &mut SystemLogAdd) {
    // 若未传日志时间，使用服务端当前时间
    if system_log.log_time == 0 {
        system_log.log_time = get_current_millisecond();
    }
    save_system_log(system_log);
}

/// 探测单个关联服务的健康接口，返回 { status, ok } 或 { error }
async fn probe_health(client: &Client, base_url: &str, path: &str) -> Value {
    let url = format!("{}{}", base_url, path);
    match client.get(&url).send().await {
        Ok(resp) => json!({
            "status": resp.status().as_u16(),
            "ok": resp.status().is_success(),
        }),
        Err(e) => json!({
            "error": e.to_string(),
        }),
    }
}

/// 探测关联服务（texhub-broadcast 等）的运行状态。
///
/// 服务地址来自配置文件（内网可达，不对外暴露），从后端主动探测后
/// 一并写入系统日志存储，前端无需访问服务状态接口。
pub async fn probe_related_services() -> Value {
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap_or_default();

    let broadcast_health_url = get_app_config("infra.broadcast_health_url");
    let mut services = json!({});
    if !broadcast_health_url.is_empty() {
        let (healthz, ready) = tokio::join!(
            probe_health(&client, &broadcast_health_url, "/health/healthz"),
            probe_health(&client, &broadcast_health_url, "/health/ready"),
        );
        services["broadcast"] = json!({
            "baseUrl": broadcast_health_url,
            "healthz": healthz,
            "ready": ready,
        });
    } else {
        services["broadcast"] = json!({
            "error": "broadcast_health_url not configured",
        });
    }
    services
}

/// 将关联服务状态合并进日志 JSON 内容（content 字段），
/// 若原始 content 不是合法 JSON 则追加到末尾。
pub fn enrich_content_with_services(content: &str, service_status: &Value) -> String {
    if let Ok(mut base) = serde_json::from_str::<Value>(content) {
        if let Some(obj) = base.as_object_mut() {
            obj.insert("serviceStatus".to_string(), service_status.clone());
            return serde_json::to_string(&base).unwrap_or_else(|_| content.to_string());
        }
    }
    // content 不是 JSON（如纯文本），追加 serviceStatus 字段
    format!(
        "{} | serviceStatus: {}",
        content,
        serde_json::to_string(service_status).unwrap_or_default()
    )
}
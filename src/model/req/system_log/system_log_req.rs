use validator::Validate;
use utoipa::ToSchema;

#[derive(serde::Deserialize, serde::Serialize, Validate, ToSchema, Default, Clone)]
#[allow(non_snake_case)]
pub struct SystemLogReq {
    /// 日志时间（毫秒时间戳，可选，默认使用服务端当前时间）
    pub log_time: Option<i64>,
    /// 日志内容
    #[validate(length(min = 1, max = 4096))]
    pub content: String,
    /// 日志来源（模块/服务名称）
    #[validate(length(min = 1, max = 128))]
    pub source: String,
    /// 日志级别（DEBUG/INFO/WARN/ERROR/FATAL）
    #[validate(length(min = 1, max = 32))]
    pub level: String,
    /// 应用 id
    #[serde(rename = "appId")]
    #[validate(length(min = 1, max = 64))]
    pub app_id: String,
}
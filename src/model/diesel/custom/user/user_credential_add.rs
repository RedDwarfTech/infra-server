use serde::Serialize;
use serde::Deserialize;
use crate::model::diesel::dolphin::dolphin_schema::*;

/// 用户登录凭据新增记录
///
/// users 表保持统一的用户身份（手机号），user_credential 表存储
/// 各登录方式的独立凭据（email/密码、短信验证码等），便于扩展。
#[derive(Insertable, Queryable, QueryableByName, Debug, Serialize, Deserialize, Default, Clone)]
#[diesel(table_name = user_credential)]
pub struct UserCredentialAdd {
    /// 关联的用户 id（对应 users.id）
    pub user_id: i64,
    /// 凭据类型（如 "email"）
    pub credential_type: String,
    /// 凭据标识（如邮箱地址）
    pub identifier: String,
    /// 凭据内容（如加盐后的密码哈希）
    pub credential: String,
    /// 加盐串，用于密码哈希
    pub salt: String,
    /// 状态：1 正常，0 失效
    pub status: i32,
    /// 应用 id
    pub app_id: String,
    /// 产品 id
    pub product_id: i32,
    /// 创建时间（毫秒时间戳）
    pub created_time: i64,
    /// 更新时间（毫秒时间戳）
    pub updated_time: i64,
}

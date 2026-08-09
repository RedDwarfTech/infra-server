-- ============================================================
-- user_credential: 用户登录凭据表
--
-- 设计说明：
--   users 表保持统一的用户身份（手机号），
--   user_credential 表存储各登录方式的独立凭据
--   （email/密码、短信验证码等），便于后续扩展
--   （如邮箱登录、第三方登录等）。
-- ============================================================
CREATE TABLE IF NOT EXISTS user_credential (
    id BIGSERIAL PRIMARY KEY,                              -- 主键
    user_id BIGINT NOT NULL DEFAULT 0,                     -- 关联的用户 id（对应 users.id）
    credential_type VARCHAR(64) NOT NULL DEFAULT '',       -- 凭据类型（如 "email"）
    identifier VARCHAR(255) NOT NULL DEFAULT '',           -- 凭据标识（如邮箱地址）
    credential VARCHAR(255) NOT NULL DEFAULT '',           -- 凭据内容（如加盐后的密码哈希）
    salt VARCHAR(64) NOT NULL DEFAULT '',                  -- 加盐串，用于密码哈希
    status INTEGER NOT NULL DEFAULT 1,                     -- 状态：1 正常，0 失效
    app_id VARCHAR(64) NOT NULL DEFAULT '',                -- 应用 id
    product_id INTEGER NOT NULL DEFAULT 0,                 -- 产品 id
    created_time BIGINT NOT NULL DEFAULT 0,                -- 创建时间（毫秒时间戳）
    updated_time BIGINT NOT NULL DEFAULT 0                 -- 更新时间（毫秒时间戳）
);

-- 按标识查询索引（登录时按 identifier + credential_type + product_id 定位凭据）
CREATE INDEX IF NOT EXISTS idx_user_credential_identifier
    ON user_credential (identifier, credential_type, product_id);
-- 按用户查询索引（用户中心展示/管理绑定凭据时使用）
CREATE INDEX IF NOT EXISTS idx_user_credential_user_id
    ON user_credential (user_id);

-- user_credential: separate user credentials table for future extension
-- users table keeps the unified user identity (phone), user_credential stores
-- per-method login credentials (email/password/sms ...)
CREATE TABLE IF NOT EXISTS user_credential (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL DEFAULT 0,
    credential_type VARCHAR(64) NOT NULL DEFAULT '',
    identifier VARCHAR(255) NOT NULL DEFAULT '',
    credential VARCHAR(255) NOT NULL DEFAULT '',
    salt VARCHAR(64) NOT NULL DEFAULT '',
    status INTEGER NOT NULL DEFAULT 1,
    app_id VARCHAR(64) NOT NULL DEFAULT '',
    product_id INTEGER NOT NULL DEFAULT 0,
    created_time BIGINT NOT NULL DEFAULT 0,
    updated_time BIGINT NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_user_credential_identifier
    ON user_credential (identifier, credential_type, product_id);
CREATE INDEX IF NOT EXISTS idx_user_credential_user_id
    ON user_credential (user_id);

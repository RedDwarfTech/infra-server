-- ============================================================
-- system_log: 系统运行日志表
--
-- 设计说明：
--   用于保存系统运行过程中产生的日志（前端上报、后端服务写入等），
--   包括日志时间、日志内容、日志来源、日志级别等信息。
-- ============================================================
CREATE TABLE IF NOT EXISTS system_log (
    id BIGSERIAL PRIMARY KEY,                              -- 主键
    log_time BIGINT NOT NULL DEFAULT 0,                    -- 日志时间（毫秒时间戳）
    content VARCHAR(4096) NOT NULL DEFAULT '',             -- 日志内容
    source VARCHAR(128) NOT NULL DEFAULT '',               -- 日志来源（如模块/服务名称）
    level VARCHAR(32) NOT NULL DEFAULT 'INFO',             -- 日志级别（DEBUG/INFO/WARN/ERROR/FATAL）
    app_id VARCHAR(64) NOT NULL DEFAULT '',                -- 应用 id
    created_time BIGINT NOT NULL DEFAULT 0,                -- 创建时间（毫秒时间戳）
    updated_time BIGINT NOT NULL DEFAULT 0                 -- 更新时间（毫秒时间戳）
);

-- 按时间查询索引（日志检索按时间范围过滤）
CREATE INDEX IF NOT EXISTS idx_system_log_log_time
    ON system_log (log_time);
-- 按来源查询索引（按模块/服务维度排查日志）
CREATE INDEX IF NOT EXISTS idx_system_log_source
    ON system_log (source);
-- 按级别查询索引（按级别过滤错误日志）
CREATE INDEX IF NOT EXISTS idx_system_log_level
    ON system_log (level);
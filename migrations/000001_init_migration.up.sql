-- 密影库 数据库初始化脚本
-- PostgreSQL 13

-- 创建用户表（如果不存在）
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    hashed_password TEXT NOT NULL,
    is_admin BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 创建资源表（如果不存在）
CREATE TABLE IF NOT EXISTS resources (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    title_en TEXT,
    description TEXT,
    resource_type TEXT NOT NULL,
    media_data BYTEA NOT NULL,
    media_type TEXT NOT NULL,
    is_local BOOLEAN NOT NULL,
    encryption_info TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    total_count INTEGER,
    has_pending_supplement BOOLEAN,
    images JSONB,
    videos JSONB,
    tags TEXT[],
    poster_image TEXT,
    author TEXT,
    source TEXT
);

-- 创建加密密钥表（如果不存在）
CREATE TABLE IF NOT EXISTS encryption_keys (
    id SERIAL PRIMARY KEY,
    resource_id INTEGER NOT NULL REFERENCES resources(id) ON DELETE CASCADE,
    key_hash TEXT NOT NULL,
    ukey_info TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 创建索引（如果不存在）
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_resources_status ON resources(status);
CREATE INDEX IF NOT EXISTS idx_resources_created_at ON resources(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_resources_media_type ON resources(media_type);
CREATE INDEX IF NOT EXISTS idx_resources_is_local ON resources(is_local);
CREATE INDEX IF NOT EXISTS idx_encryption_keys_resource_id ON encryption_keys(resource_id);

-- 插入初始数据（如果admin用户不存在）
DO $$
BEGIN
    IF NOT EXISTS (SELECT FROM users WHERE username = 'admin') THEN
        INSERT INTO users (username, hashed_password, is_admin) VALUES 
        ('admin', '$2a$10$N9qo8uLOickgx2ZMRZoMyeIjZAgcfl7p92ldGxad68LJZdL17lhWy', TRUE);
    END IF;
END
$$;
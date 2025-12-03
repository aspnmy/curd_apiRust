-- 初始迁移文件 - 创建通用表结构

-- 通用数据表结构
CREATE TABLE IF NOT EXISTS common_data (
    id SERIAL PRIMARY KEY,                      -- 主键ID
    table_name VARCHAR(255) NOT NULL,           -- 表名，用于区分不同类型的数据
    datainfos JSONB NOT NULL DEFAULT '{}'::jsonb, -- 通用JSON数据存储
    is_rols VARCHAR(50) DEFAULT 'users',        -- 权限标识，如users、admin等
    is_del BOOLEAN DEFAULT FALSE,               -- 逻辑删除标识，true表示已删除
    is_date TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP, -- 更新时间
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP, -- 创建时间
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP -- 实际更新时间
);

-- 创建索引以提高查询性能
CREATE INDEX IF NOT EXISTS idx_common_data_table_name ON common_data(table_name);
CREATE INDEX IF NOT EXISTS idx_common_data_is_del ON common_data(is_del);
CREATE INDEX IF NOT EXISTS idx_common_data_is_rols ON common_data(is_rols);
CREATE INDEX IF NOT EXISTS idx_common_data_datainfos ON common_data USING GIN(datainfos);

-- 创建触发器函数，自动更新is_date和updated_at字段
CREATE OR REPLACE FUNCTION update_common_data_timestamps()
RETURNS TRIGGER AS $$
BEGIN
    NEW.is_date = CURRENT_TIMESTAMP;
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 为common_data表添加更新触发器
CREATE TRIGGER update_common_data_trigger
BEFORE UPDATE ON common_data
FOR EACH ROW
EXECUTE FUNCTION update_common_data_timestamps();

-- 插入初始admin用户数据
INSERT INTO common_data (table_name, datainfos, is_rols)
VALUES (
    'users', 
    '{"username": "admin", "hashed_password": "$2a$10$N9qo8uLOickgx2ZMRZoMyeIjZAgcfl7p92ldGxad68LJZdL17lhWy", "is_admin": true}'::jsonb,
    'admin'
) ON CONFLICT DO NOTHING;

-- 迁移脚本：为各表添加datainfo字段，支持JSON数据存储

-- 为users表添加datainfo字段
ALTER TABLE users ADD COLUMN IF NOT EXISTS datainfo JSONB DEFAULT '{}';

-- 为resources表添加datainfo字段
ALTER TABLE resources ADD COLUMN IF NOT EXISTS datainfo JSONB DEFAULT '{}';

-- 为encryption_keys表添加datainfo字段
ALTER TABLE encryption_keys ADD COLUMN IF NOT EXISTS datainfo JSONB DEFAULT '{}';

-- 创建索引，加速JSON字段查询
CREATE INDEX IF NOT EXISTS idx_users_datainfo ON users USING GIN(datainfo);
CREATE INDEX IF NOT EXISTS idx_resources_datainfo ON resources USING GIN(datainfo);
CREATE INDEX IF NOT EXISTS idx_encryption_keys_datainfo ON encryption_keys USING GIN(datainfo);

-- 更新现有记录的datainfo字段，确保默认值正确
UPDATE users SET datainfo = '{}' WHERE datainfo IS NULL;
UPDATE resources SET datainfo = '{}' WHERE datainfo IS NULL;
UPDATE encryption_keys SET datainfo = '{}' WHERE datainfo IS NULL;

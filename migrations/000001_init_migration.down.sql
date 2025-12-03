-- 回滚初始迁移 - 删除通用表结构

-- 删除触发器
DROP TRIGGER IF EXISTS update_common_data_trigger ON common_data;

-- 删除触发器函数
DROP FUNCTION IF EXISTS update_common_data_timestamps;

-- 删除索引
DROP INDEX IF EXISTS idx_common_data_table_name;
DROP INDEX IF EXISTS idx_common_data_is_del;
DROP INDEX IF EXISTS idx_common_data_is_rols;
DROP INDEX IF EXISTS idx_common_data_datainfos;

-- 删除通用数据表
DROP TABLE IF EXISTS common_data;

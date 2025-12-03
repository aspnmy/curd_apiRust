-- 回滚初始迁移 - 删除创建的表结构

-- 删除资源表
DROP TABLE IF EXISTS resources;

-- 删除加密密钥表
DROP TABLE IF EXISTS encryption_keys;

-- 删除用户表
DROP TABLE IF EXISTS users;

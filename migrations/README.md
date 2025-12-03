# 数据库迁移说明

## 迁移文件命名规则

迁移文件采用以下命名格式：
```
{序号}_{描述}.sql
```

- `{序号}`：两位数的序号，从01开始，用于确定迁移顺序
- `{描述}`：迁移内容的简要描述，使用下划线分隔单词

## 迁移文件内容

每个迁移文件应包含两个部分：
1. **UP** 迁移：用于应用迁移（创建表、添加字段等）
2. **DOWN** 迁移：用于回滚迁移（删除表、移除字段等）

## 如何运行迁移

迁移会在应用启动时自动运行，无需手动执行。

## 如何创建新的迁移

1. 在 `migrations` 目录下创建一个新的SQL文件，使用正确的命名格式
2. 编写UP和DOWN迁移脚本
3. 确保迁移脚本可以正确执行

## 注意事项

1. **不要修改已应用的迁移文件**：如果迁移已经被应用到数据库，修改它会导致迁移失败
2. **确保DOWN迁移可以正确回滚UP迁移**：这样在需要时可以安全地回滚
3. **测试迁移脚本**：在生产环境应用之前，务必在开发环境测试
4. **备份数据库**：在应用新的迁移之前，建议备份数据库

## 常见问题

### 迁移失败："migration 1 was previously applied but has been modified"

这个错误表示迁移文件1已经被应用到数据库，但后来被修改了。解决方法：

1. 如果是开发环境，可以删除数据库并重新创建
2. 如果是生产环境，需要创建一个新的迁移文件来修复问题，而不是修改已有的文件

### 迁移顺序问题

确保迁移文件的序号正确，迁移会按照序号顺序执行。

## 示例迁移文件

```sql
-- 01_create_users_table.sql

-- UP
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE,
    hashed_password VARCHAR(100) NOT NULL,
    is_admin BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- DOWN
DROP TABLE IF EXISTS users;
```
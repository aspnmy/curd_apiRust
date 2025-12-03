# 通用API接口使用文档

## 1. API概述

本项目提供了一套基于Rust和Axum的通用CRUD API服务，支持多实例部署、读写分离和动态SQL生成。所有API请求均使用HTTP POST方法，请求体为JSON格式，响应也为JSON格式。

## 2. 健康检查接口

### 2.1 接口描述

用于检查服务的健康状态，包括服务本身和数据库连接状态。

### 2.2 接口信息

```
GET /health
```

### 2.3 响应示例

```json
{
  "status": "healthy",
  "service_id": "crud-write-01",
  "service_role": "write",
  "version": "0.1.0",
  "database_status": "healthy",
  "started_at": "2025-12-03T10:00:00Z"
}
```

## 3. 通用CRUD接口

### 3.1 接口描述

支持对任意数据的增删改查操作，通过操作类型和请求参数动态生成SQL查询。

### 3.2 接口信息

```
POST /api/common
```

### 3.3 简化的API端点

为了方便使用，提供了简化的API端点，对应增删改查四个操作：

```
# 添加记录
POST /api/add

# 查询记录
POST /api/check

# 更新记录
POST /api/update

# 软删除记录
POST /api/isdel
```

这些简化端点的请求体与通用请求格式相同，但操作类型由路径决定。

## 4. 批量操作接口

### 4.1 接口描述

支持批量执行多个CRUD操作，提高处理效率。

### 4.2 接口信息

```
POST /api/common/batch
```

## 5. 请求格式

### 5.1 通用请求格式

所有请求都使用POST方法，请求体为JSON格式，包含操作类型、表名、数据和条件等信息。

```json
{
  "operation": "add", // add, check, update, isdel
  "table_name": "users", // 逻辑表名
  "data": { /* 操作数据 */ }, // 操作数据，用于 add 和 update
  "where_conditions": [ /* 查询条件 */ ], // 查询条件，用于 check, update 和 isdel
  "fields": [ /* 查询字段 */ ], // 查询字段，用于 check
  "soft_delete_config": { /* 软删除配置 */ } // 软删除配置，用于 isdel
}
```

### 5.2 操作类型

| 操作类型 | 描述 | 适用端点 |
|---------|------|---------|
| `add` | 添加记录 | `/api/common`, `/api/add` |
| `check` | 查询记录 | `/api/common`, `/api/check` |
| `update` | 更新记录 | `/api/common`, `/api/update` |
| `isdel` | 软删除记录 | `/api/common`, `/api/isdel` |

### 5.3 查询条件格式

```json
[
  {
    "field": "username", // 字段名
    "operator": "=", // 操作符
    "value": "test_user" // 值
  }
]
```

支持的操作符：
- `=` 等于
- `>` 大于
- `<` 小于
- `>=` 大于等于
- `<=` 小于等于
- `!=` 不等于
- `LIKE` 模糊匹配
- `IN` 在列表中
- `NOT IN` 不在列表中
- `IS NULL` 为空
- `IS NOT NULL` 不为空

### 5.4 软删除配置格式

```json
{
  "field": "is_del", // 软删除字段名
  "value": "true" // 软删除值
}
```

### 5.5 批量请求格式

```json
{
  "requests": [
    { /* 第一个请求 */ },
    { /* 第二个请求 */ }
  ]
}
```

## 6. 响应格式

### 6.1 通用响应格式

```json
{
  "success": true, // 操作是否成功
  "message": "操作成功", // 响应消息
  "data": [ /* 响应数据 */ ], // 响应数据，用于 check 操作
  "affected_rows": 1, // 受影响的行数，用于 add, update 和 isdel 操作
  "total": 10, // 总记录数，用于 check 操作
  "encryption_info": null, // 加密信息，如果有加密操作
  "service_id": "crud-write-01", // 服务ID
  "service_role": "write" // 服务角色
}
```

### 6.2 批量响应格式

```json
{
  "results": [ /* 每个请求的响应结果 */ ],
  "success_count": 2, // 成功的请求数
  "failure_count": 0, // 失败的请求数
  "service_id": "crud-write-01", // 服务ID
  "service_role": "write" // 服务角色
}
```

## 7. 使用示例

### 7.1 添加记录

#### 请求

```json
{
  "operation": "add",
  "table_name": "users",
  "data": {
    "username": "test_user",
    "email": "test@example.com",
    "password": "hashed_password"
  }
}
```

#### 响应

```json
{
  "success": true,
  "message": "添加成功",
  "data": {
    "id": 1,
    "table_name": "users",
    "datainfos": {
      "username": "test_user",
      "email": "test@example.com",
      "password": "hashed_password"
    },
    "is_rols": "users",
    "is_del": false,
    "is_date": "2025-12-03T10:00:00Z",
    "created_at": "2025-12-03T10:00:00Z",
    "updated_at": "2025-12-03T10:00:00Z"
  },
  "affected_rows": 1,
  "total": null,
  "encryption_info": null,
  "service_id": "crud-write-01",
  "service_role": "write"
}
```

### 7.2 查询记录

#### 请求

```json
{
  "operation": "check",
  "table_name": "users",
  "where_conditions": [
    {
      "field": "username",
      "operator": "=",
      "value": "test_user"
    }
  ],
  "fields": ["id", "datainfos", "is_rols"]
}
```

#### 响应

```json
{
  "success": true,
  "message": "查询成功",
  "data": [
    {
      "id": 1,
      "datainfos": {
        "username": "test_user",
        "email": "test@example.com",
        "password": "hashed_password"
      },
      "is_rols": "users"
    }
  ],
  "affected_rows": null,
  "total": 1,
  "encryption_info": null,
  "service_id": "crud-read-01",
  "service_role": "read"
}
```

### 7.3 更新记录

#### 请求

```json
{
  "operation": "update",
  "table_name": "users",
  "data": {
    "email": "new_email@example.com"
  },
  "where_conditions": [
    {
      "field": "username",
      "operator": "=",
      "value": "test_user"
    }
  ]
}
```

#### 响应

```json
{
  "success": true,
  "message": "更新成功",
  "data": [
    {
      "id": 1,
      "table_name": "users",
      "datainfos": {
        "username": "test_user",
        "email": "new_email@example.com",
        "password": "hashed_password"
      },
      "is_rols": "users",
      "is_del": false,
      "is_date": "2025-12-03T10:05:00Z",
      "created_at": "2025-12-03T10:00:00Z",
      "updated_at": "2025-12-03T10:05:00Z"
    }
  ],
  "affected_rows": 1,
  "total": null,
  "encryption_info": null,
  "service_id": "crud-write-01",
  "service_role": "write"
}
```

### 7.4 软删除记录

#### 请求

```json
{
  "operation": "isdel",
  "table_name": "users",
  "soft_delete_config": {
    "field": "is_del",
    "value": "true"
  },
  "where_conditions": [
    {
      "field": "username",
      "operator": "=",
      "value": "test_user"
    }
  ]
}
```

#### 响应

```json
{
  "success": true,
  "message": "软删除成功",
  "data": [
    {
      "id": 1,
      "table_name": "users",
      "datainfos": {
        "username": "test_user",
        "email": "new_email@example.com",
        "password": "hashed_password"
      },
      "is_rols": "users",
      "is_del": true,
      "is_date": "2025-12-03T10:10:00Z",
      "created_at": "2025-12-03T10:00:00Z",
      "updated_at": "2025-12-03T10:10:00Z"
    }
  ],
  "affected_rows": 1,
  "total": null,
  "encryption_info": null,
  "service_id": "crud-write-01",
  "service_role": "write"
}
```

### 7.5 批量操作

#### 请求

```json
{
  "requests": [
    {
      "operation": "add",
      "table_name": "users",
      "data": {
        "username": "user1",
        "email": "user1@example.com",
        "password": "hashed_password1"
      }
    },
    {
      "operation": "add",
      "table_name": "users",
      "data": {
        "username": "user2",
        "email": "user2@example.com",
        "password": "hashed_password2"
      }
    }
  ]
}
```

#### 响应

```json
{
  "results": [
    {
      "success": true,
      "message": "添加成功",
      "data": {
        "id": 2,
        "table_name": "users",
        "datainfos": {
          "username": "user1",
          "email": "user1@example.com",
          "password": "hashed_password1"
        },
        "is_rols": "users",
        "is_del": false,
        "is_date": "2025-12-03T10:15:00Z",
        "created_at": "2025-12-03T10:15:00Z",
        "updated_at": "2025-12-03T10:15:00Z"
      },
      "affected_rows": 1,
      "total": null,
      "encryption_info": null,
      "service_id": "crud-write-01",
      "service_role": "write"
    },
    {
      "success": true,
      "message": "添加成功",
      "data": {
        "id": 3,
        "table_name": "users",
        "datainfos": {
          "username": "user2",
          "email": "user2@example.com",
          "password": "hashed_password2"
        },
        "is_rols": "users",
        "is_del": false,
        "is_date": "2025-12-03T10:15:00Z",
        "created_at": "2025-12-03T10:15:00Z",
        "updated_at": "2025-12-03T10:15:00Z"
      },
      "affected_rows": 1,
      "total": null,
      "encryption_info": null,
      "service_id": "crud-write-01",
      "service_role": "write"
    }
  ],
  "success_count": 2,
  "failure_count": 0,
  "service_id": "crud-write-01",
  "service_role": "write"
}
```

## 8. 错误处理

### 8.1 错误响应格式

当请求失败时，API会返回错误响应，包含错误代码、错误消息和详细信息。

```json
{
  "success": false,
  "message": "无效的操作类型",
  "data": null,
  "affected_rows": null,
  "total": null,
  "encryption_info": null,
  "service_id": "crud-write-01",
  "service_role": "write"
}
```

### 8.2 常见错误

| 错误类型 | 描述 | 可能的原因 |
|---------|------|-----------|
| `无效的操作类型` | 操作类型不支持 | 操作类型不是 add, check, update 或 isdel |
| `只读角色不允许写操作` | 服务角色不允许此操作 | 读取节点尝试执行写操作 |
| `数据库迁移失败` | 数据库迁移执行失败 | 迁移脚本错误或数据库连接问题 |
| `无效的条件格式` | 查询条件格式错误 | 条件字段缺失或格式不正确 |
| `软删除配置不能为空` | 软删除操作缺少配置 | 软删除操作没有提供 soft_delete_config |
| `数据必须是对象` | 操作数据格式错误 | add 或 update 操作的数据不是对象 |

## 9. 服务角色

API支持三种服务角色，实现读写分离：

| 角色 | 描述 | 允许的操作 |
|-----|------|-----------|
| `read` | 只读角色 | 仅允许执行 check 操作 |
| `write` | 只写角色 | 允许执行 add, update, isdel 操作 |
| `mixed` | 混合角色 | 允许执行所有操作 |

## 10. 多节点部署

API支持多节点部署，实现高可用和负载均衡：

- **写入节点**：负责处理写操作和数据库迁移，建议部署1-2个实例
- **读取节点**：负责处理读操作，建议部署多个实例，根据负载情况调整
- **健康检查**：所有节点提供 `/health` 端点，用于健康检查

## 11. 通用表结构

项目使用单一表结构支持所有数据类型，无需修改表结构：

```sql
CREATE TABLE common_data (
    id SERIAL PRIMARY KEY,                      -- 主键ID
    table_name VARCHAR(255) NOT NULL,           -- 逻辑表名，用于区分不同类型的数据
    datainfos JSONB NOT NULL DEFAULT '{}'::jsonb, -- 通用JSON数据存储
    is_rols VARCHAR(50) DEFAULT 'users',        -- 权限标识，如users、admin等
    is_del BOOLEAN DEFAULT FALSE,               -- 逻辑删除标识，true表示已删除
    is_date TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP, -- 更新时间
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP, -- 创建时间
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP -- 实际更新时间
);
```

## 12. 配置说明

API服务通过环境变量进行配置，主要配置项包括：

| 变量名 | 描述 | 默认值 |
|-------|------|-------|
| `SERVER_HOST` | 服务器地址 | `0.0.0.0` |
| `SERVER_PORT` | 服务器端口 | `8000` |
| `DATABASE_URL` | 数据库连接URL | `postgres://user:password@localhost:5432/secret_gallery` |
| `SERVICE_ROLE` | 服务角色 | `mixed` |
| `SERVICE_ID` | 服务ID | `crud-01` |
| `ALLOWED_TABLES` | 允许操作的逻辑表名白名单 | `users,resources,encryption_keys` |
| `RUN_MIGRATIONS` | 是否运行数据库迁移 | `true` |
| `MIGRATION_STRATEGY` | 数据库迁移策略 | `strict` |

## 13. 安全注意事项

1. **API密钥**：建议使用API网关或负载均衡器配置API密钥验证
2. **JWT认证**：使用JWT进行用户认证，确保请求来源合法
3. **HTTPS**：生产环境中建议启用HTTPS，保护数据传输安全
4. **参数验证**：所有请求参数都需要进行验证，防止注入攻击
5. **加密存储**：敏感数据应加密存储，如密码、密钥等
6. **权限控制**：根据用户角色控制访问权限，实现最小权限原则
7. **日志记录**：记录所有API请求和响应，便于审计和调试
8. **速率限制**：配置速率限制，防止恶意请求和DDoS攻击

## 14. 开发和测试

### 14.1 本地开发

```bash
# 安装依赖
cargo install sqlx-cli

# 运行数据库
docker run -d -p 5432:5432 -e POSTGRES_USER=user -e POSTGRES_PASSWORD=password -e POSTGRES_DB=secret_gallery postgres:14-alpine

# 运行迁移
sqlx migrate run

# 启动服务
cargo run
```

### 14.2 测试API

可以使用curl、Postman或其他API测试工具测试API：

```bash
# 健康检查
curl -X GET http://localhost:8000/health

# 添加记录
curl -X POST http://localhost:8000/api/common -H "Content-Type: application/json" -d '{"operation": "add", "table_name": "users", "data": {"username": "test_user", "email": "test@example.com", "password": "hashed_password"}}'

# 查询记录
curl -X POST http://localhost:8000/api/common -H "Content-Type: application/json" -d '{"operation": "check", "table_name": "users", "where_conditions": [{"field": "username", "operator": "=", "value": "test_user"}]}'
```

## 15. 监控和日志

API服务使用Tracing库进行日志记录，支持不同级别的日志：

- **INFO**：记录服务启动、配置加载、数据库连接等信息
- **WARN**：记录警告信息，如配置缺失、连接超时等
- **ERROR**：记录错误信息，如数据库迁移失败、请求处理失败等

可以通过环境变量控制日志级别：

```bash
export RUST_LOG=info
cargo run
```

## 16. 性能优化

1. **数据库索引**：为常用查询字段创建索引，提高查询性能
2. **连接池**：使用数据库连接池，减少连接建立和关闭的开销
3. **缓存**：对于频繁查询的数据，使用缓存提高响应速度
4. **异步处理**：使用异步编程模型，提高并发处理能力
5. **批量操作**：使用批量操作API，减少网络请求次数
6. **分页查询**：对于大量数据，使用分页查询，减少内存占用
7. **负载均衡**：使用负载均衡器分发请求，提高系统吞吐量

## 17. 扩展和定制

API设计支持扩展和定制，可以通过以下方式扩展功能：

1. **添加新的操作类型**：在 `CommonService::execute` 方法中添加新的操作类型处理
2. **定制响应格式**：修改 `CommonResponse` 结构体，添加或修改响应字段
3. **添加新的API端点**：在 `routes.rs` 中添加新的路由和处理函数
4. **定制数据库模型**：修改 `common_data` 表结构，添加新的字段
5. **添加新的中间件**：在 `create_router` 函数中添加新的中间件，如认证、日志、限流等

## 18. 版本控制

API版本控制建议使用以下方式：

1. **URL路径版本控制**：如 `/v1/api/common`
2. **请求头版本控制**：使用 `Accept-Version` 或 `X-API-Version` 请求头
3. **媒体类型版本控制**：使用自定义媒体类型，如 `application/vnd.crud-api.v1+json`

## 19. 部署和运维

### 19.1 Docker部署

```bash
# 构建镜像
docker build -t curd_api_rust .

# 运行容器
docker run -d --name curd_api_rust -p 8000:8000 curd_api_rust
```

### 19.2 Docker Compose部署

```yaml
version: '3.8'

services:
  api-write:
    container_name: curd_api_rust_write
    image: curd_api_rust:latest
    environment:
      - SERVICE_ROLE=write
      - SERVICE_ID=crud-write-01
      - DATABASE_URL=${DATABASE_URL}
      - ALLOWED_TABLES=users,resources,encryption_keys
      - RUN_MIGRATIONS=true
      - MIGRATION_STRATEGY=ignore
    ports:
      - "7981:8000"
    restart: unless-stopped
  
  api-read:
    container_name: curd_api_rust_read
    image: curd_api_rust:latest
    environment:
      - SERVICE_ROLE=read
      - SERVICE_ID=crud-read-01
      - DATABASE_URL=${DATABASE_URL}
      - ALLOWED_TABLES=users,resources,encryption_keys
      - RUN_MIGRATIONS=false
    ports:
      - "7982:8000"
    restart: unless-stopped
```

### 19.3 Kubernetes部署

可以使用Kubernetes部署API服务，实现高可用和自动扩缩容：

- 使用Deployment部署服务实例
- 使用Service暴露服务
- 使用Ingress配置负载均衡
- 使用ConfigMap管理配置
- 使用Secret管理敏感信息

## 20. 总结

本项目提供了一套基于Rust和Axum的通用CRUD API服务，支持多实例部署、读写分离和动态SQL生成。通过单一表结构支持所有数据类型，无需修改表结构，实现了高度的灵活性和可扩展性。

API设计遵循RESTful原则，提供了简洁易用的接口，支持批量操作和多种服务角色。项目使用现代化的技术栈，具有高性能、高可靠性和高安全性的特点，适合构建各种规模的应用系统。

## 21. 联系方式

如有问题或建议，请创建Issue或提交Pull Request。

- **项目地址**：https://github.com/aspnmy/curd_api_rust
- **文档地址**：https://github.com/aspnmy/curd_api_rust/tree/main/docs
- **许可证**：MIT

## 22. 更新日志

### v0.1.0 (2025-12-03)

- 初始版本
- 支持通用CRUD操作
- 支持批量操作
- 支持多节点部署和读写分离
- 支持通用表结构
- 支持软删除
- 支持健康检查
- 支持Docker和Docker Compose部署

### v0.2.0 (计划中)

- 添加认证和授权功能
- 添加API版本控制
- 添加速率限制
- 添加缓存支持
- 添加更详细的日志和监控
- 添加更多测试用例
- 优化性能和安全性

## 23. 附录

### 23.1 环境变量列表

| 变量名 | 描述 | 默认值 | 必需 |
|-------|------|-------|------|
| `SERVER_HOST` | 服务器地址 | `0.0.0.0` | 否 |
| `SERVER_PORT` | 服务器端口 | `8000` | 否 |
| `HTTPS` | 是否启用HTTPS | `false` | 否 |
| `DATABASE_URL` | 数据库连接URL | `postgres://user:password@localhost:5432/secret_gallery` | 是 |
| `DATABASE_MAX_CONNECTIONS` | 数据库最大连接数 | `10` | 否 |
| `DATABASE_MIN_CONNECTIONS` | 数据库最小连接数 | `2` | 否 |
| `JWT_SECRET` | JWT密钥 | `your_secret_key` | 否 |
| `JWT_EXPIRES_IN` | JWT过期时间（秒） | `3600` | 否 |
| `JWT_REFRESH_IN` | JWT刷新时间（秒） | `86400` | 否 |
| `ENCRYPTION_ALGORITHM` | 加密算法 | `aes-256-gcm` | 否 |
| `ENCRYPTION_KEY_LENGTH` | 密钥长度 | `32` | 否 |
| `ENCRYPTION_ITERATIONS` | 迭代次数 | `100000` | 否 |
| `SERVICE_ROLE` | 服务角色 | `mixed` | 否 |
| `SERVICE_ID` | 服务ID | `crud-01` | 否 |
| `ALLOWED_TABLES` | 允许操作的逻辑表名白名单 | `users,resources,encryption_keys` | 否 |
| `RUN_MIGRATIONS` | 是否运行数据库迁移 | `true` | 否 |
| `MIGRATION_STRATEGY` | 数据库迁移策略 | `strict` | 否 |

### 23.2 数据类型支持

API支持以下数据类型：

- 字符串 (String)
- 数字 (Integer, Float)
- 布尔值 (Boolean)
- 对象 (Object)
- 数组 (Array)
- null

所有数据都会被存储为JSON格式，使用PostgreSQL的JSONB类型。

### 23.3 字符集和编码

API使用UTF-8字符集，支持国际化和多语言。

### 23.4 时区

所有时间字段都使用UTC时区，格式为ISO 8601，如 `2025-12-03T10:00:00Z`。

### 23.5 最佳实践

1. **使用HTTPS**：生产环境中建议启用HTTPS，保护数据传输安全
2. **合理配置服务角色**：根据负载情况合理配置读写节点数量
3. **使用批量操作**：对于大量数据操作，使用批量API提高效率
4. **优化查询条件**：使用索引字段作为查询条件，提高查询性能
5. **合理设置数据库连接池**：根据并发量设置合适的连接池大小
6. **定期备份数据库**：定期备份数据库，防止数据丢失
7. **监控服务状态**：监控服务健康状态，及时发现和解决问题
8. **使用负载均衡**：使用负载均衡器分发请求，提高系统吞吐量
9. **实现自动扩缩容**：根据负载情况自动调整服务实例数量
10. **定期更新依赖**：定期更新依赖包，修复安全漏洞和性能问题
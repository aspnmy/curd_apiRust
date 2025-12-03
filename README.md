# CRUD API Service

一个基于 Rust 和 Axum 的通用 CRUD API 服务，支持多实例部署、读写分离和动态 SQL 生成。

## 架构设计

### 核心功能

- **通用 CRUD 操作**：支持对任意表的增删改查操作
- **动态 SQL 生成**：根据请求参数动态生成 SQL 查询
- **参数化查询**：防止 SQL 注入，提高安全性
- **服务角色支持**：支持 read/write/mixed 三种角色，实现读写分离
- **批量处理**：支持批量增删改查操作
- **软删除**：支持配置软删除字段
- **条件查询**：支持复杂的条件查询
- **JSONB 存储**：支持 PostgreSQL JSONB 类型存储

### 技术栈

- **语言**：Rust 2024
- **Web 框架**：Axum 0.7.9
- **数据库**：PostgreSQL 14+
- **ORM**：SQLx
- **异步运行时**：Tokio
- **序列化**：Serde
- **日志**：Tracing
- **配置管理**：Dotenvy

## 部署方式

### Docker 部署

```bash
# 构建镜像
docker build -t crudapi .

# 运行容器 - 写角色
docker run -d \
  --name crudapi-write \
  -p 8000:8000 \
  -e SERVICE_ROLE=write \
  -e SERVICE_ID=crudapi-write-01 \
  -e DATABASE_URL=postgres://user:password@postgres:5432/secret_gallery \
  -e ALLOWED_TABLES=users,resources,encryption_keys \
  crudapi

# 运行容器 - 读角色
docker run -d \
  --name crudapi-read \
  -p 8001:8000 \
  -e SERVICE_ROLE=read \
  -e SERVICE_ID=crudapi-read-01 \
  -e DATABASE_URL=postgres://user:password@postgres:5432/secret_gallery \
  -e ALLOWED_TABLES=users,resources \
  crudapi
```

### Docker Compose 部署

```yaml
version: '3.8'

services:
  # API服务
  api:
    build: 
      context: .
      dockerfile: Dockerfile
    container_name: crud_api
    environment:
      - SERVER_HOST=0.0.0.0
      - SERVER_PORT=8000
      - HTTPS=false
      - DATABASE_URL=postgres://crud_user:crud_password@localhost:5432/crud_db
      - DATABASE_MAX_CONNECTIONS=10
      - DATABASE_MIN_CONNECTIONS=2
      - JWT_SECRET=your_secure_jwt_secret_key_here
      - JWT_EXPIRES_IN=3600
      - JWT_REFRESH_IN=86400
      - ENCRYPTION_ALGORITHM=aes-256-gcm
      - ENCRYPTION_KEY_LENGTH=32
      - ENCRYPTION_ITERATIONS=100000
      - SERVICE_ROLE=mixed
      - SERVICE_ID=crud-01
      - ALLOWED_TABLES=users,resources,encryption_keys
    ports:
      - "8000:8000"
    restart: unless-stopped
```

或者使用项目根目录的 docker-compose.yml 文件：

```bash
docker-compose up -d
```

## 配置说明

### 环境变量

| 变量名 | 描述 | 默认值 |
|--------|------|--------|
| `SERVER_HOST` | 服务器地址 | `0.0.0.0` |
| `SERVER_PORT` | 服务器端口 | `8000` |
| `HTTPS` | 是否启用 HTTPS | `false` |
| `DATABASE_URL` | 数据库连接 URL | `postgres://user:password@localhost:5432/secret_gallery` |
| `DATABASE_MAX_CONNECTIONS` | 数据库最大连接数 | `10` |
| `DATABASE_MIN_CONNECTIONS` | 数据库最小连接数 | `2` |
| `JWT_SECRET` | JWT 密钥 | `your_secret_key` |
| `JWT_EXPIRES_IN` | JWT 过期时间（秒） | `3600` |
| `JWT_REFRESH_IN` | JWT 刷新时间（秒） | `86400` |
| `ENCRYPTION_ALGORITHM` | 加密算法 | `aes-256-gcm` |
| `ENCRYPTION_KEY_LENGTH` | 密钥长度 | `32` |
| `ENCRYPTION_ITERATIONS` | 迭代次数 | `100000` |
| `SERVICE_ROLE` | 服务角色（read/write/mixed） | `mixed` |
| `SERVICE_ID` | 服务 ID | `crud-01` |
| `ALLOWED_TABLES` | 允许操作的表名白名单，逗号分隔 | `users,resources,encryption_keys` |

## API 端点

### 健康检查

```
GET /health
```

### 通用 CRUD 端点

#### 通用请求格式

所有请求都使用 POST 方法，请求体为 JSON 格式，包含操作类型、表名、数据和条件等信息。

```json
{
  "operation": "add", // add, check, update, isdel
  "table_name": "users", // 表名
  "data": { /* 操作数据 */ }, // 操作数据，用于 add 和 update
  "where_conditions": [ /* 查询条件 */ ], // 查询条件，用于 check, update 和 isdel
  "fields": [ /* 查询字段 */ ], // 查询字段，用于 check
  "soft_delete_config": { /* 软删除配置 */ } // 软删除配置，用于 isdel
}
```

#### 通用 CRUD API

```
POST /api/common
```

#### 批量操作 API

```
POST /api/common/batch

请求体：
{
  "requests": [
    { /* 第一个请求 */ },
    { /* 第二个请求 */ }
  ]
}
```

#### 简化的 API 端点

为了方便使用，提供了简化的 API 端点，对应增删改查四个操作：

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

## 服务角色

### Read 角色

- 仅允许执行 SELECT 操作
- 适用于读密集型应用
- 可水平扩展，提高读性能

### Write 角色

- 允许执行 INSERT/UPDATE/DELETE 操作
- 适用于写密集型应用
- 可配置为单实例或多实例部署

### Mixed 角色

- 允许执行所有 CRUD 操作
- 适用于开发环境或小型部署

## 开发指南

### 本地开发

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

### 构建

```bash
# 构建开发版本
cargo build

# 构建发布版本
cargo build --release
```

### 测试

```bash
# 运行单元测试
cargo test

# 运行集成测试
cargo test -- --ignored
```

### 代码检查

```bash
# 检查语法错误
cargo check

# 格式化代码
cargo fmt

# 运行 clippy
cargo clippy
```

## 贡献指南

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 打开 Pull Request

## 许可证

MIT

## 联系信息

如有问题或建议，请创建 Issue 或提交 Pull Request。

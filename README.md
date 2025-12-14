# CURD API Rust Service

一个基于 Rust 和 Axum 的通用 CRUD API 服务，支持多实例部署、读写分离、动态 SQL 生成和插件系统。

## 架构设计

### 核心功能

- **通用 CRUD 操作**：支持对任意数据的增删改查操作
- **动态 SQL 生成**：根据请求参数动态生成 SQL 查询
- **参数化查询**：防止 SQL 注入，提高安全性
- **服务角色支持**：支持 read/write/mixed 三种角色，实现读写分离
- **批量处理**：支持批量增删改查操作
- **软删除**：内置逻辑删除功能，通过 is_del 字段控制
- **条件查询**：支持复杂的条件查询
- **JSONB 存储**：使用 PostgreSQL JSONB 类型存储通用数据
- **多节点部署**：支持多个服务节点，实现高可用和负载均衡
- **通用表结构**：单一表结构支持所有数据类型，无需修改表结构
- **灵活权限控制**：支持 users/admin 等多种权限标识
- **自动时间更新**：自动更新 is_date 字段，记录最后更新时间
- **插件系统**：支持动态加载和执行插件，实现功能扩展
  - **image2base64**：图片转Base64插件
  - **image2dicom**：图片转DICOM插件

### 技术栈

- **语言**：Rust 2024
- **Web 框架**：Axum 0.7.9
- **数据库**：PostgreSQL 14+
- **ORM**：SQLx
- **异步运行时**：Tokio
- **序列化**：Serde
- **日志**：Tracing
- **配置管理**：Dotenvy
- **HTTP客户端**：Reqwest (用于插件系统)
- **Base64编码**：base64 crate (用于图片处理)
- **随机数生成**：rand crate (用于生成唯一ID)

## 部署方式

### Docker 部署

```bash
# 构建镜像
docker build -t curd_api_rust .

# 运行容器 - 写入节点
docker run -d \
  --name curd_api_rust_write \
  -p 7981:8000 \
  -e SERVICE_ROLE=write \
  -e SERVICE_ID=crud-write-01 \
  -e DATABASE_URL=postgres://user:password@postgres:5432/secret_gallery \
  -e SQL_TABLE=users,resources,encryption_keys \
  -e RUN_MIGRATIONS=true \
  -e MIGRATION_STRATEGY=ignore \
  ghcr.io/aspnmy/curd_apirust:latest

# 运行容器 - 读取节点
docker run -d \
  --name curd_api_rust_read \
  -p 7982:8000 \
  -e SERVICE_ROLE=read \
  -e SERVICE_ID=crud-read-01 \
  -e DATABASE_URL=postgres://user:password@postgres:5432/secret_gallery \
  -e SQL_TABLE=users,resources \
  -e RUN_MIGRATIONS=false \
  ghcr.io/aspnmy/curd_apirust:latest
```

### Docker Compose 部署

```yaml
version: '3.8'

services:
  # 写入节点 - 负责处理写操作和数据库迁移
  api-write:
    container_name: curd_api_rust_write
    image: ghcr.io/aspnmy/curd_apirust:latest
    environment:
      # 服务器配置
      - SERVER_HOST=0.0.0.0
      - SERVER_PORT=8000
      - HTTPS=false
      
      # 数据库配置
      - DATABASE_URL=${DATABASE_URL}
      - DATABASE_MAX_CONNECTIONS=30
      - DATABASE_MIN_CONNECTIONS=2
      
      # JWT配置
      - JWT_SECRET=${JWT_PWD}
      - JWT_EXPIRES_IN=3600
      - JWT_REFRESH_IN=86400
      
      # 加密配置
      - ENCRYPTION_ALGORITHM=aes-256-gcm
      - ENCRYPTION_KEY_LENGTH=32
      - ENCRYPTION_ITERATIONS=100000
      
      # 服务配置
      - SERVICE_ROLE=write
      - SERVICE_ID=crud-write-01
      - SQL_TABLE=users,resources,encryption_keys
      - RUN_MIGRATIONS=true
      - MIGRATION_STRATEGY=${MIGRATION_STRATEGY}
      
    ports:
      - "7981:8000"
    restart: unless-stopped
  
  # 读取节点 - 负责处理读操作
  api-read:
    container_name: curd_api_rust_read
    image: ghcr.io/aspnmy/curd_apirust:latest
    environment:
      # 服务器配置
      - SERVER_HOST=0.0.0.0
      - SERVER_PORT=8000
      - HTTPS=false
      
      # 数据库配置
      - DATABASE_URL=${DATABASE_URL}
      - DATABASE_MAX_CONNECTIONS=30
      - DATABASE_MIN_CONNECTIONS=2
      
      # JWT配置
      - JWT_SECRET=${JWT_PWD}
      - JWT_EXPIRES_IN=3600
      - JWT_REFRESH_IN=86400
      
      # 加密配置
      - ENCRYPTION_ALGORITHM=aes-256-gcm
      - ENCRYPTION_KEY_LENGTH=32
      - ENCRYPTION_ITERATIONS=100000
      
      # 服务配置
      - SERVICE_ROLE=read
      - SERVICE_ID=crud-read-01
      - SQL_TABLE=users,resources,encryption_keys
      - RUN_MIGRATIONS=false
      
    ports:
      - "7982:8000"
    restart: unless-stopped
```

使用方式：

```bash
# 创建.env文件，添加必要的环境变量
cat > .env << EOF
DATABASE_URL=postgres://user:password@postgres:5432/secret_gallery
JWT_PWD=your_secure_jwt_secret_key_here
MIGRATION_STRATEGY=ignore
EOF

# 启动服务
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
| `SQL_TABLE` | 允许操作的表名，仅支持单表 | `common_data` |
| `RUN_MIGRATIONS` | 是否运行数据库迁移 | `true` |
| `MIGRATION_STRATEGY` | 数据库迁移策略，可选值：repair（修复）、ignore（忽略）、strict（严格） | `strict` |
| `ISRULE` | 是否启用插件系统 | `false` |
| `ENABLED_PLUGINS` | 启用的插件列表，逗号分隔 | `image2base64,image2dicom` |

## API 端点

### 健康检查

```
GET /health
```

### 通用 CRUD 端点

#### 通用请求格式

所有请求都使用 POST 方法，请求体为 JSON 格式，包含操作类型、文件类型、数据和条件等信息。

```json
{
  "operation": "add", // add, check, update, isdel
  "file_type": "image", // 逻辑文件类型
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

## 通用表结构

### 表结构设计

```sql
CREATE TABLE common_data (
    id SERIAL PRIMARY KEY,                      -- 主键ID
    file_type VARCHAR(255) NOT NULL,            -- 逻辑文件类型，用于区分不同类型的数据
    datainfos JSONB NOT NULL DEFAULT '{}'::jsonb, -- 通用JSON数据存储
    is_rols VARCHAR(50) DEFAULT 'users',        -- 权限标识，如users、admin等
    is_del BOOLEAN DEFAULT FALSE,               -- 逻辑删除标识，true表示已删除
    is_date TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP, -- 更新时间
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP, -- 创建时间
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP -- 实际更新时间
);
```

### 字段说明

- **id**：主键ID，自动递增
- **file_type**：逻辑文件类型，用于区分不同类型的数据
- **datainfos**：通用JSON数据存储，支持任意键值对
- **is_rols**：权限标识，如users、admin等
- **is_del**：逻辑删除标识，true表示已删除
- **is_date**：更新时间，自动更新
- **created_at**：创建时间，仅在创建时设置
- **updated_at**：实际更新时间，自动更新

### 使用示例

#### 添加记录

```json
{
  "operation": "add",
  "file_type": "image",
  "data": {
    "file_name": "test.jpg",
    "file_size": 1024,
    "file_content": "base64_encoded_image_data"
  }
}
```

#### 查询记录

```json
{
  "operation": "check",
  "file_type": "image",
  "where_conditions": [
    {
      "field": "file_name",
      "operator": "LIKE",
      "value": "%test%"
    }
  ]
}
```

#### 更新记录

```json
{
  "operation": "update",
  "file_type": "image",
  "data": {
    "file_description": "Updated description"
  },
  "where_conditions": [
    {
      "field": "file_id",
      "operator": "=",
      "value": "file_123456789"
    }
  ]
}
```

#### 软删除记录

```json
{
  "operation": "isdel",
  "file_type": "image",
  "soft_delete_config": {
    "field": "is_del",
    "value": "true"
  },
  "where_conditions": [
    {
      "field": "file_id",
      "operator": "=",
      "value": "file_123456789"
    }
  ]
}
```

## 服务角色

### Read 角色

- 仅允许执行 SELECT 操作
- 适用于读密集型应用
- 可水平扩展，提高读性能
- 跳过数据库迁移

### Write 角色

- 允许执行 INSERT/UPDATE/DELETE 操作
- 适用于写密集型应用
- 负责数据库迁移
- 可配置为单实例或多实例部署

### Mixed 角色

- 允许执行所有 CRUD 操作
- 适用于开发环境或小型部署
- 执行数据库迁移

## 多节点部署

### 部署架构

1. **写入节点**：
   - 建议部署 1-2 个实例
   - 设置 `SERVICE_ROLE=write`
   - 设置 `RUN_MIGRATIONS=true`（仅一个节点）
   - 设置 `MIGRATION_STRATEGY=ignore`（生产环境）或 `repair`（首次部署）
   - 负责处理写操作和数据库迁移

2. **读取节点**：
   - 建议部署多个实例，根据负载情况调整
   - 设置 `SERVICE_ROLE=read`
   - 设置 `RUN_MIGRATIONS=false`
   - 负责处理读操作
   - 可通过负载均衡器进行负载分配

### 负载均衡

- **Nginx**：配置反向代理，将请求分发到不同节点
- **HAProxy**：高性能负载均衡器，支持 TCP 和 HTTP 协议
- **Kubernetes**：使用 Service 和 Ingress 进行负载均衡

### 健康检查

所有节点提供 `/health` 端点，用于健康检查。

## 开发指南

### 本地开发

```bash
# 安装依赖
cargo install sqlx-cli

# 运行数据库
docker run -d -p 5432:5432 -e POSTGRES_USER=user -e POSTGRES_PASSWORD=password -e POSTGRES_DB=secret_gallery postgres:14-alpine

# 复制环境变量模板
cp .env.example .env

# 编辑.env文件，配置数据库连接等信息

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

## 插件系统

### 插件开发

插件系统基于 `RulePlugin` trait 设计，所有插件都需要实现这个 trait。

```rust
pub trait RulePlugin {
    fn name(&self) -> &'static str;
    fn init(&mut self) -> Result<()> { Ok(()) }
    fn execute(
        &self,
        file_type: &str,
        data: serde_json::Value,
    ) -> BoxFuture<'_, Result<serde_json::Value>>;
}
```

### 内置插件

#### image2base64

将图片转换为 Base64 编码格式。

- 支持多种图片输入格式
- 自动检测图片 MIME 类型
- 生成完整的 Base64 URL

#### image2dicom

将图片转换为 DICOM 格式。

- 调用外部 DICOM 转换服务
- 支持多种图片输入格式
- 生成 DICOM 文件路径和 Base64 编码

### 插件配置

插件可以通过环境变量进行配置：

```bash
# 启用插件系统
ISRULE=true

# 启用特定插件
ENABLED_PLUGINS=image2base64,image2dicom
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

# 连接到WSL2环境

## 1. 获取WSL2 IP地址

在WSL2终端中运行以下命令获取IP地址：

```bash
ip addr show eth0 | grep -oP '(?<=inet\s)\d+(\.\d+){3}'
```

或者使用更简单的命令：

```bash
hostname -I | cut -d' ' -f1
```

## 2. 配置PostgreSQL

### 2.1 在WSL2中安装PostgreSQL

```bash
sudo apt update
sudo apt install postgresql postgresql-contrib
```

### 2.2 启动PostgreSQL服务

```bash
sudo systemctl start postgresql
sudo systemctl enable postgresql
```

### 2.3 创建数据库和用户

```bash
# 进入PostgreSQL命令行
sudo -u postgres psql

# 创建数据库
CREATE DATABASE crud_db;

# 创建用户
CREATE USER crud_user WITH PASSWORD 'crud_password';

# 授予权限
GRANT ALL PRIVILEGES ON DATABASE crud_db TO crud_user;
ALTER USER crud_user CREATEDB;

# 退出
\q
```

### 2.4 允许远程连接

编辑PostgreSQL配置文件：

```bash
sudo nano /etc/postgresql/15/main/postgresql.conf
```

将`listen_addresses`设置为`*`：

```
listen_addresses = '*'
```

编辑pg_hba.conf文件：

```bash
sudo nano /etc/postgresql/15/main/pg_hba.conf
```

添加以下行以允许远程连接：

```
host    all             all             0.0.0.0/0               md5
```

重启PostgreSQL服务：

```bash
sudo systemctl restart postgresql
```

## 3. 配置项目连接

### 3.1 更新.env文件

编辑.env文件，将`wsl2_ip`替换为你的WSL2 IP地址：

```bash
nano .env
```

修改DATABASE_URL：

```
DATABASE_URL=postgres://crud_user:crud_password@<your_wsl2_ip>:5432/crud_db
```

### 3.2 运行数据库迁移

```bash
cargo sqlx migrate run
```

### 3.3 启动项目

```bash
cargo run
```

## 4. 在Windows中访问项目

项目将在`http://localhost:8000`上运行，你可以在Windows浏览器中访问。

## 5. 使用Docker Compose连接到WSL2

如果你想使用Docker Compose并连接到WSL2中的PostgreSQL，可以修改docker-compose.yml文件：

```yaml
version: '3.8'

services:
  # API服务
  api:
    build: .
    container_name: crud_api
    environment:
      - SERVER_HOST=0.0.0.0
      - SERVER_PORT=8000
      - HTTPS=false
      - DATABASE_URL=postgres://crud_user:crud_password@host.docker.internal:5432/crud_db
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
    ports:
      - "8000:8000"
    networks:
      - crud_network
    restart: unless-stopped

networks:
  crud_network:
    driver: bridge
```

然后在WSL2中运行：

```bash
docker compose up -d
```

注意：使用`host.docker.internal`可以让Docker容器访问Windows主机上的服务，包括WSL2中的服务（如果WSL2的PostgreSQL已经配置为允许远程连接）。

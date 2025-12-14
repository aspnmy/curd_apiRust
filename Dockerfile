# 第一阶段：后端构建阶段
FROM docker.io/library/rust:1.91.1-slim AS backend-builder

# 安装构建依赖
RUN apt-get update --allow-releaseinfo-change && apt-get install -y --no-install-recommends \
    build-essential \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# 设置工作目录
WORKDIR /app

# 复制依赖文件
COPY Cargo.toml Cargo.lock ./

# 复制源代码
COPY src ./src
COPY migrations ./migrations

# 构建应用
RUN cargo build --release

# # 第二阶段：Caddy构建阶段
# FROM docker.io/library/caddy:alpine AS caddy-builder

# # 设置工作目录
# WORKDIR /app/ui

# # 复制前端源代码
# COPY ui ./

# # 复制Caddyfile配置
# COPY ui/Caddyfile /etc/caddy/Caddyfile

# 第三阶段：运行时镜像
FROM docker.io/library/debian:13.2-slim

# 安装依赖
RUN apt-get update --allow-releaseinfo-change && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# 设置工作目录
WORKDIR /app

# 从后端构建阶段复制可执行文件
COPY --from=backend-builder /app/target/release/curd_api_rust ./

# # 从Caddy构建阶段复制Caddy二进制文件
# COPY --from=caddy-builder /usr/bin/caddy /usr/bin/caddy

# # 从Caddy构建阶段复制前端UI文件
# COPY --from=caddy-builder /app/ui /usr/share/caddy

# # 从Caddy构建阶段复制Caddy配置文件
# COPY --from=caddy-builder /etc/caddy/Caddyfile /etc/caddy/Caddyfile

# 复制环境变量示例文件
COPY .env.example ./

# # 复制入口脚本
# COPY sh/start.sh ./

# RUN chmod +x start.sh

# 暴露端口
EXPOSE 8000
#EXPOSE 80

# 运行启动脚本
CMD ["./curd_api_rust"]

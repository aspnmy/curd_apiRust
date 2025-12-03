FROM rust:1.79.0-slim-buster AS builder

# 设置工作目录
WORKDIR /app

# 复制依赖文件
COPY Cargo.toml Cargo.lock ./

# 复制源代码
COPY src ./src
COPY migrations ./migrations

# 构建应用
RUN cargo build --release

# 创建运行时镜像
FROM debian:bullseye-slim

# 安装依赖
RUN apt-get update --allow-releaseinfo-change && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 设置工作目录
WORKDIR /app

# 从构建阶段复制可执行文件
COPY --from=builder /app/target/release/crudapi ./

# 复制环境变量示例文件
COPY .env.example ./

# 暴露端口
EXPOSE 8000

# 运行应用
CMD ["./crudapi"]

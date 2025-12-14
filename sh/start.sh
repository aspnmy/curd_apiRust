#!/bin/bash
# 启动脚本：同时运行caddy和后端服务

# 启动Caddy服务
caddy run --config /etc/caddy/Caddyfile --adapter caddyfile &

# 启动后端服务
./curd_api_rust
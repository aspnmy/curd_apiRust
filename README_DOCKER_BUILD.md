# 在WSL2环境中只进行Docker镜像编译

## 1. 准备工作

### 1.1 确保WSL2已安装并配置

确保你已经安装了WSL2，并且已经配置了Ubuntu或其他Linux发行版。

### 1.2 在WSL2中安装Docker

```bash
# 更新包列表
sudo apt update

# 安装必要的包
sudo apt install -y apt-transport-https ca-certificates curl software-properties-common

# 添加Docker的GPG密钥
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -

# 添加Docker仓库
sudo add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable"

# 安装Docker
sudo apt update
sudo apt install -y docker-ce docker-ce-cli containerd.io

# 启动Docker服务
sudo systemctl start docker
sudo systemctl enable docker

# 将当前用户添加到docker组（避免每次使用sudo）
sudo usermod -aG docker $USER
```

**注意**：添加用户到docker组后，需要重新登录WSL2才能生效。

### 1.3 安装Docker Compose

```bash
# 下载Docker Compose
sudo curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose

# 赋予执行权限
sudo chmod +x /usr/local/bin/docker-compose

# 验证安装
docker-compose --version
```

## 2. 编译Docker镜像

### 2.1 进入项目目录

```bash
cd /path/to/your/project/crud_apiRust
```

### 2.2 编译Docker镜像

使用docker-compose编译镜像：

```bash
docker-compose build
```

或者直接使用docker命令编译：

```bash
docker build -t crud_api .
```

## 3. 运行编译后的镜像

### 3.1 使用docker-compose运行

```bash
docker-compose up -d
```

### 3.2 使用docker命令运行

```bash
docker run -d --name crud_api -p 8000:8000 crud_api
```

## 4. 查看容器状态

```bash
# 查看所有容器
docker ps -a

# 查看容器日志
docker logs crud_api

# 进入容器内部
docker exec -it crud_api /bin/bash
```

## 5. 停止和删除容器

```bash
# 停止容器
docker-compose down
# 或者
docker stop crud_api

# 删除容器
docker rm crud_api
```

## 6. 注意事项

1. **数据库连接**：
   - 本配置将数据库连接地址设置为`localhost`，因此运行容器时需要确保宿主机上有可用的PostgreSQL服务
   - 或者你可以在运行时通过环境变量覆盖数据库连接地址

2. **环境变量**：
   - 你可以在`.env`文件中配置环境变量，这些变量会被docker-compose读取
   - 或者在运行容器时通过`-e`参数设置环境变量

3. **镜像优化**：
   - 如果你需要优化镜像大小，可以考虑使用多阶段构建（当前Dockerfile已经使用了多阶段构建）
   - 可以使用`docker build --no-cache`避免使用缓存，强制重新构建

4. **网络配置**：
   - 本配置使用了默认的网络模式，容器可以通过`localhost`访问宿主机上的服务
   - 如果你需要更复杂的网络配置，可以在docker-compose.yml中添加网络配置

## 7. 仅构建镜像不运行

如果你只想构建镜像而不运行容器，可以使用以下命令：

```bash
# 使用docker-compose
docker-compose build

# 使用docker命令
docker build -t crud_api .
```

## 8. 推送镜像到仓库（可选）

```bash
# 登录到Docker Hub
docker login

# 标记镜像
docker tag crud_api your-dockerhub-username/crud_api:latest

# 推送镜像
docker push your-dockerhub-username/crud_api:latest
```

## 9. 从仓库拉取镜像（可选）

```bash
docker pull your-dockerhub-username/crud_api:latest
```

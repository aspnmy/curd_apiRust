// UI配置文件
// 这里可以配置API地址等全局参数

// 动态配置API地址，适配不同环境
// 在Docker容器环境中，会自动使用服务名称通信
// 在本地环境中，使用localhost地址
const getApiBaseUrl = () => {
    // 检查当前域名或主机名，判断环境
    const hostname = window.location.hostname;
    
    // 如果是本地开发环境
    if (hostname === 'localhost' || hostname === '127.0.0.1') {
        return 'http://localhost:8000/api'; // 本地开发环境，使用本地后端服务
    } 
    // 如果是Docker容器环境（通过容器名称访问）
    else if (hostname.includes('curd_api_rust_ui') || hostname.includes('test_ui')) {
        return 'http://api-write:8000/api'; // Docker Compose环境，使用服务名称访问后端
    } 
    // 如果是远程部署环境
    else {
        return 'http://10.168.3.165:7891/api'; // 远程部署环境，使用配置的远程地址
    }
};

const UI_CONFIG = {
    // API基础URL - 根据环境动态生成
    API_BASE_URL: getApiBaseUrl()
};
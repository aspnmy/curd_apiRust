use std::net::SocketAddr;
use std::sync::Arc;

use axum::{serve};
use tracing::info;
use dotenvy::dotenv;

use crate::service::common::CommonService;
use crate::database::{init_database_pool, run_migrations};
use crate::api::routes::create_router;
use crate::config::AppConfig;

mod config;
mod database;
mod service;
mod api;

#[tokio::main]
async fn main() {
    // 加载环境变量
    dotenv().ok();
    
    // 初始化日志
    tracing_subscriber::fmt::init();
    
    // 加载配置
    let config = AppConfig::from_env().expect("无法加载配置");
    config.validate().expect("配置验证失败");
    
    info!("服务配置: {:?}", config);
    
    // 初始化数据库连接池
    let db_pool = init_database_pool(&config)
        .await
        .expect("无法初始化数据库连接池");
    
    // 运行数据库迁移
    run_migrations(&db_pool)
        .await
        .expect("无法运行数据库迁移");
    
    // 创建服务实例
    let common_service = CommonService::new(db_pool.clone(), config.clone());
    let common_service = Arc::new(common_service);
    
    // 构建路由
    let app = create_router(
        common_service,
        config.clone()
    );
    
    // 配置服务器地址
    let addr = SocketAddr::from((
        config.server.host.parse::<std::net::IpAddr>().expect("无效的服务器地址"),
        config.server.port
    ));
    
    info!("服务器正在启动，监听地址: {}, 服务ID: {}, 服务角色: {}", 
          addr, 
          config.service.id, 
          config.service.role);
    
    // 启动服务器
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("无法绑定地址");
    
    info!("服务器正在运行，监听地址: {}", listener.local_addr().unwrap());
    
    serve(listener, app)
        .await
        .expect("服务器启动失败");
}

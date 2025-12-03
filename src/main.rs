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
    
    // 根据环境变量决定是否运行数据库迁移
    if std::env::var("RUN_MIGRATIONS").unwrap_or("true".to_string()) == "true" {
        // 获取迁移策略
        let migration_strategy = std::env::var("MIGRATION_STRATEGY").unwrap_or("strict".to_string());
        
        // 运行数据库迁移
        match run_migrations(&db_pool).await {
            Ok(_) => {
                info!("数据库迁移成功");
            },
            Err(e) => {
                match migration_strategy.as_str() {
                    "repair" => {
                        // 修复策略：尝试修复迁移记录
                        info!("数据库迁移失败，尝试修复迁移记录: {:?}", e);
                        // 这里可以添加修复逻辑，例如使用sqlx::migrate!().repair()
                        // 注意：修复操作需要谨慎使用，建议在开发环境测试后再在生产环境使用
                        info!("迁移修复功能尚未实现，请手动修复迁移记录");
                    },
                    "ignore" => {
                        // 忽略策略：跳过迁移错误，继续运行服务
                        info!("数据库迁移失败，忽略错误继续运行: {:?}", e);
                    },
                    _ => {
                        // 严格策略：迁移失败时退出
                        panic!("无法运行数据库迁移: {:?}", e);
                    }
                }
            }
        };
    } else {
        info!("跳过数据库迁移，RUN_MIGRATIONS环境变量设置为false");
    }
    
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

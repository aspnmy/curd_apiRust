use std::fs::OpenOptions;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;

use axum::serve;
use dotenvy::dotenv;
use tracing::info;
use tracing_subscriber::{fmt, layer::{Layer, SubscriberExt}, util::SubscriberInitExt, filter::LevelFilter};

use crate::api::routes::create_router;
use crate::config::AppConfig;
use crate::database::{init_database_pool, run_migrations};
use crate::service::common::CommonService;

mod api;
mod config;
mod database;
mod service;

/// 设置日志配置
fn setup_logging(config: &AppConfig) {
    // 创建日志目录
    let log_dir = Path::new(&config.log_path);
    if !log_dir.exists() {
        std::fs::create_dir_all(log_dir).expect("无法创建日志目录");
    }

    // 设置日志级别
    let log_level = if config.debug {
        LevelFilter::DEBUG
    } else {
        LevelFilter::INFO
    };

    // 创建日志文件
    let log_file_path = log_dir.join("app.log");
    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(log_file_path)
        .expect("无法打开日志文件");

    // 配置日志格式
    let file_layer = fmt::layer()
        .with_writer(std::sync::Arc::new(log_file))
        .with_ansi(false)
        .with_level(true)
        .with_target(true);

    let console_layer = fmt::layer()
        .with_ansi(true)
        .with_level(true)
        .with_target(true);

    // 初始化日志
    tracing_subscriber::registry()
        .with(file_layer.with_filter(log_level))
        .with(console_layer.with_filter(log_level))
        .init();
}

#[tokio::main]
async fn main() {
    // 加载环境变量
    dotenv().ok();

    // 加载配置
    let config = AppConfig::from_env().expect("无法加载配置");
    config.validate().expect("配置验证失败");

    // 初始化日志
    setup_logging(&config);

    info!("服务配置: {:?}", config);

    // 初始化数据库连接池
    let db_pool = init_database_pool(&config)
        .await
        .expect("无法初始化数据库连接池");

    // 根据环境变量决定是否运行数据库迁移
    let should_run_migrations = std::env::var("RUN_MIGRATIONS").unwrap_or("true".to_string()) == "true";
    
    if should_run_migrations {
        // 获取迁移策略
        let migration_strategy = 
            std::env::var("MIGRATION_STRATEGY").unwrap_or("strict".to_string());

        info!("准备执行数据库迁移，策略: {}", migration_strategy);
        
        // 根据迁移策略决定是否执行迁移
        match migration_strategy.as_str() {
            "ignore" | "repair" => {
                // 忽略或修复策略：执行迁移
                info!("执行数据库迁移");
                let result = run_migrations(&db_pool).await;

                match result {
                    Ok(_) => {
                        info!("数据库迁移成功");
                    }
                    Err(e) => {
                        info!("数据库迁移失败: {:?}", e);
                        
                        if migration_strategy == "ignore" {
                            // 忽略策略：跳过迁移错误，继续运行服务
                            info!("忽略迁移错误，继续运行服务");
                        } else {
                            // 修复策略：尝试修复迁移记录
                            info!("尝试修复迁移记录，但修复功能尚未实现");
                            // 注意：修复操作需要谨慎使用，建议在开发环境测试后再在生产环境使用
                        }
                    }
                };
            }
            "strict" => {
                // 严格策略：检查迁移状态，如果需要迁移则执行，否则跳过
                info!("严格模式：检查迁移状态");
                
                // 执行迁移，严格模式下迁移失败会导致程序退出
                let result = run_migrations(&db_pool).await;
                
                match result {
                    Ok(_) => {
                        info!("数据库迁移成功");
                    }
                    Err(e) => {
                        info!("数据库迁移失败: {:?}", e);
                        // 严格策略：迁移失败时退出
                        panic!("无法运行数据库迁移: {:?}", e);
                    }
                };
            }
            _ => {
                // 未知策略：使用默认的严格模式
                info!("未知迁移策略，使用默认的严格模式");
                let result = run_migrations(&db_pool).await;
                
                if let Err(e) = result {
                    panic!("无法运行数据库迁移: {:?}", e);
                }
                
                info!("数据库迁移成功");
            }
        }
    } else {
        info!("跳过数据库迁移，RUN_MIGRATIONS环境变量设置为false");
    }

    // 创建服务实例
    let common_service = CommonService::new(db_pool.clone(), config.clone());
    let common_service = Arc::new(common_service);

    // 构建路由
    let app = create_router(common_service, config.clone());

    // 配置服务器地址
    let addr = SocketAddr::from((
        config
            .server
            .host
            .parse::<std::net::IpAddr>()
            .expect("无效的服务器地址"),
        config.server.port,
    ));

    info!(
        "服务器正在启动，监听地址: {}, 服务ID: {}, 服务角色: {}",
        addr, config.service.id, config.service.role
    );

    // 启动服务器
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("无法绑定地址");

    info!(
        "服务器正在运行，监听地址: {}",
        listener.local_addr().unwrap()
    );

    serve(listener, app).await.expect("服务器启动失败");
}

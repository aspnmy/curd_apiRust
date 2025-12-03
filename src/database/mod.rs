use anyhow::Result;
use sqlx::PgPool;
use tracing::{error, info};

use crate::config::AppConfig;

/// 数据库连接池类型别名
pub type DatabasePool = PgPool;

/// 数据库错误类型
#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    /// 连接错误
    #[error("数据库连接错误: {0}")]
    ConnectionError(#[from] sqlx::Error),
    /// 迁移错误
    #[error("数据库迁移错误: {0}")]
    MigrationError(String),
    /// 事务错误
    #[error("数据库事务错误: {0}")]
    TransactionError(#[from] anyhow::Error),
}

/// 初始化数据库连接池
pub async fn init_database_pool(config: &AppConfig) -> Result<DatabasePool, DatabaseError> {
    info!("初始化数据库连接池，配置: {:?}", config.database);

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .min_connections(config.database.min_connections)
        .connect(&config.database.url)
        .await
        .map_err(|e| DatabaseError::ConnectionError(e))?;

    info!("数据库连接池初始化成功");
    Ok(pool)
}

/// 运行数据库迁移
pub async fn run_migrations(pool: &DatabasePool) -> Result<(), DatabaseError> {
    info!("运行数据库迁移");

    sqlx::migrate!().run(pool).await.map_err(|e| {
        error!("数据库迁移失败: {:?}", e);
        DatabaseError::MigrationError(e.to_string())
    })?;

    info!("数据库迁移完成");
    Ok(())
}

/// 健康检查
#[allow(dead_code)]
pub async fn health_check(pool: &DatabasePool) -> Result<(), DatabaseError> {
    info!("执行数据库健康检查");

    sqlx::query("SELECT 1")
        .fetch_one(pool)
        .await
        .map_err(|e| DatabaseError::ConnectionError(e))?;

    info!("数据库健康检查通过");
    Ok(())
}

pub mod models;

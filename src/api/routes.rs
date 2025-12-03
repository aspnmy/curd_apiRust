use axum::{Extension, Router, http::StatusCode, routing::get, routing::post};
use std::sync::Arc;

use crate::api::handlers::common_handlers;
use crate::config::AppConfig;
use crate::service::common::CommonService;

/// 404处理程序
async fn not_found_handler() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "404 Not Found - CRUD API服务")
}

/// 创建API路由
pub fn create_router(common_service: Arc<CommonService>, config: AppConfig) -> Router {
    // 创建路由
    Router::new()
        // 健康检查
        .route("/health", get(common_handlers::health_check))
        // API路由组
        .nest("/api", {
            Router::new()
                // 通用CRUD API
                .route("/common", post(common_handlers::handle_common_request))
                .route("/common/batch", post(common_handlers::handle_batch_request))
                // 简化的API端点，对应增删改查四个操作
                .route("/add", post(common_handlers::handle_common_request))
                .route("/check", post(common_handlers::handle_common_request))
                .route("/update", post(common_handlers::handle_common_request))
                .route("/isdel", post(common_handlers::handle_common_request))
        })
        // 添加404处理，使用axum::routing::any处理所有未匹配的请求
        .fallback(get(not_found_handler))
        // 添加中间件
        .layer(Extension(common_service))
        .layer(Extension(config.clone()))
}

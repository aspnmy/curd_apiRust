use axum::{Extension, Router, http::StatusCode, routing::get, routing::post, middleware};
use axum::middleware::Next;
use std::sync::Arc;
use axum::{body::Body, extract::Request, http::{header, HeaderValue, Method}, response::IntoResponse};

use crate::api::handlers::common_handlers;
use crate::config::AppConfig;
use crate::service::common::CommonService;

/// 自定义CORS中间件
async fn custom_cors(
    Extension(config): Extension<AppConfig>,
    req: Request<Body>,
    next: Next
) -> impl IntoResponse {
    let origin = req
        .headers()
        .get(header::ORIGIN)
        .map(|i| i.to_str().map(|s| s.to_owned()));

    let is_options = req.method() == Method::OPTIONS;
    let mut res = if is_options {
        let mut res = "".into_response();
        res.headers_mut().insert(
            header::ACCESS_CONTROL_MAX_AGE,
            HeaderValue::from_static("9999999"),
        );
        res
    } else {
        next.run(req).await
    };

    let headers = res.headers_mut();

    if let Some(Ok(origin)) = origin {
        headers.insert(
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_str(&origin).unwrap(),
        );
    }

    headers.insert(
        header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
        HeaderValue::from_static("true"),
    );

    headers.insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("*"),
    );

    // 添加Access-Control-Allow-Headers头，使用配置的值
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_str(&config.cors_allow_headers).unwrap(),
    );

    // 添加安全头，防止MIME类型嗅探
    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );

    // 添加缓存控制头，建议浏览器不要缓存API响应
    headers.insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("no-store, no-cache, must-revalidate, max-age=0"),
    );

    // 确保JSON响应包含正确的charset
    if let Some(content_type) = headers.get(header::CONTENT_TYPE) {
        if content_type.to_str().unwrap_or("").starts_with("application/json") {
            headers.insert(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/json; charset=utf-8"),
            );
        }
    }

    res
}

/// 404处理程序
async fn not_found_handler() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "404 Not Found - CRUD API服务")
}



/// 创建API路由
pub fn create_router(common_service: Arc<CommonService>, config: AppConfig) -> Router {
    // 创建API路由组
    let mut api_router = Router::new()
        // 通用CRUD API
        .route("/common", post(common_handlers::handle_common_request))
        .route("/common/batch", post(common_handlers::handle_batch_request))
        // 简化的API端点，对应增删改查四个操作
        .route("/add", post(common_handlers::handle_common_request))
        .route("/check", post(common_handlers::handle_common_request))
        .route("/update", post(common_handlers::handle_common_request))
        .route("/isdel", post(common_handlers::handle_common_request))
        // 基于file_type的API端点，结构为 /api/common/{file_type}/{add、check、update、isdel}
        .route("/common/:file_type/add", post(common_handlers::handle_file_type_request))
        .route("/common/:file_type/check", post(common_handlers::handle_file_type_request))
        .route("/common/:file_type/update", post(common_handlers::handle_file_type_request))
        .route("/common/:file_type/isdel", post(common_handlers::handle_file_type_request));

    // 只有在调试模式下才添加日志API
    if config.debug {
        api_router = api_router.route("/logs", get(common_handlers::get_logs));
    }

    // 创建基础路由
    let mut router = Router::new()
        // 健康检查
        .route("/health", get(common_handlers::health_check))
        // API路由组
        .nest("/api", api_router)
        // 添加CORS中间件
        .route_layer(middleware::from_fn(custom_cors))
        // 添加其他中间件
        .layer(Extension(common_service))
        .layer(Extension(config.clone()));

    // 添加404处理，使用axum::routing::any处理所有未匹配的请求
    router = router.fallback(get(not_found_handler));
    
    router
}

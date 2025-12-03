use axum::{http::StatusCode, Json, Extension};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::service::common::CommonService;
use crate::database::models::common::{CommonRequest, CommonResponse, BatchRequest, BatchResponse, HealthResponse};

/// 处理通用操作请求
pub async fn handle_common_request(
    Extension(common_service): Extension<Arc<CommonService>>,
    Json(request): Json<CommonRequest>,
) -> (StatusCode, Json<CommonResponse>) {
    match common_service.execute(request).await {
        Ok(response) => (StatusCode::OK, Json(response)),
        Err(e) => {
            tracing::error!("处理通用请求失败: {:?}", e);
            let error_response = CommonResponse {
                success: false,
                message: format!("{}", e),
                data: None,
                affected_rows: None,
                total: None,
                encryption_info: None,
                service_id: common_service.config.service.id.clone(),
                service_role: common_service.config.service.role.clone(),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        }
    }
}

/// 处理批量操作请求
pub async fn handle_batch_request(
    Extension(common_service): Extension<Arc<CommonService>>,
    Json(batch_request): Json<BatchRequest>,
) -> (StatusCode, Json<BatchResponse>) {
    let mut results = Vec::new();
    let mut success_count = 0;
    let mut failure_count = 0;

    for request in batch_request.requests {
        match common_service.execute(request).await {
            Ok(response) => {
                results.push(response);
                success_count += 1;
            },
            Err(e) => {
                tracing::error!("处理批量请求中的单个请求失败: {:?}", e);
                let error_response = CommonResponse {
                    success: false,
                    message: format!("{}", e),
                    data: None,
                    affected_rows: None,
                    total: None,
                    encryption_info: None,
                    service_id: common_service.config.service.id.clone(),
                    service_role: common_service.config.service.role.clone(),
                };
                results.push(error_response);
                failure_count += 1;
            }
        }
    }

    let batch_response = BatchResponse {
        results,
        success_count,
        failure_count,
        service_id: common_service.config.service.id.clone(),
        service_role: common_service.config.service.role.clone(),
    };

    (StatusCode::OK, Json(batch_response))
}

/// 健康检查处理函数
pub async fn health_check(
    Extension(common_service): Extension<Arc<CommonService>>,
) -> (StatusCode, Json<HealthResponse>) {
    match common_service.get_health().await {
        Ok(health) => (StatusCode::OK, Json(health)),
        Err(e) => {
            tracing::error!("健康检查失败: {:?}", e);
            let error_health = HealthResponse {
                status: "unhealthy".to_string(),
                service_id: common_service.config.service.id.clone(),
                service_role: common_service.config.service.role.clone(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                database_status: "unhealthy".to_string(),
                started_at: common_service.started_at.clone(),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_health))
        }
    }
}

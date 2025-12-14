use axum::{Extension, Json, http::StatusCode};
use std::sync::Arc;
use tracing::{info, error, debug};

use crate::database::models::common::{
    BatchRequest, BatchResponse, CommonRequest, CommonResponse, HealthResponse,
};
use crate::service::common::CommonService;

/// 处理通用操作请求
pub async fn handle_common_request(
    Extension(common_service): Extension<Arc<CommonService>>,
    Json(request): Json<CommonRequest>,
) -> (StatusCode, Json<CommonResponse>) {
    // 记录请求信息
    info!("接收通用请求 - 操作类型: {}, 表名: {}, 服务ID: {}", 
        request.operation, request.table_name, common_service.config.service.id);
    debug!("请求详情: {:?}", request);
    
    let result = common_service.execute(request).await;
    
    match result {
        Ok(response) => {
            // 记录成功响应
            info!("处理通用请求成功 - 服务ID: {}, 影响行数: {:?}, 状态: {}", 
                response.service_id, 
                response.affected_rows, 
                if response.success { "成功" } else { "失败" });
            debug!("响应详情: {:?}", response);
            (StatusCode::OK, Json(response))
        },
        Err(e) => {
            // 记录错误响应
            error!("处理通用请求失败: {:?}", e);
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
    // 记录批量请求信息
    info!("接收批量请求 - 请求数量: {}, 服务ID: {}", 
        batch_request.requests.len(), common_service.config.service.id);
    debug!("批量请求详情: {:?}", batch_request);
    
    let mut results = Vec::new();
    let mut success_count = 0;
    let mut failure_count = 0;

    for (index, request) in batch_request.requests.into_iter().enumerate() {
        info!("处理批量请求中的第 {} 个请求 - 操作类型: {}, 表名: {}", 
            index + 1, request.operation, request.table_name);
        
        match common_service.execute(request).await {
            Ok(response) => {
                results.push(response);
                success_count += 1;
                info!("批量请求中的第 {} 个请求处理成功", index + 1);
            }
            Err(e) => {
                error!("处理批量请求中的第 {} 个请求失败: {:?}", index + 1, e);
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
    
    // 记录批量请求处理结果
    info!("批量请求处理完成 - 成功: {}, 失败: {}, 服务ID: {}", 
        batch_response.success_count, batch_response.failure_count, batch_response.service_id);
    debug!("批量响应详情: {:?}", batch_response);

    (StatusCode::OK, Json(batch_response))
}

/// 健康检查处理函数
pub async fn health_check(
    Extension(common_service): Extension<Arc<CommonService>>,
) -> (StatusCode, Json<HealthResponse>) {
    info!("接收健康检查请求 - 服务ID: {}", common_service.config.service.id);
    
    match common_service.get_health().await {
        Ok(health) => {
            info!("健康检查成功 - 状态: healthy");
            (StatusCode::OK, Json(health))
        },
        Err(e) => {
            error!("健康检查失败: {:?}", e);
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

/// 获取日志内容
pub async fn get_logs(
    Extension(common_service): Extension<Arc<CommonService>>,
) -> (StatusCode, Json<serde_json::Value>) {
    info!("接收日志请求 - 服务ID: {}", common_service.config.service.id);
    
    // 检查是否启用调试模式
    if !common_service.config.debug {
        error!("尝试在非调试模式下访问日志API - 服务ID: {}", common_service.config.service.id);
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "调试模式未启用"})));
    }
    
    // 读取日志文件
    let log_file_path = std::path::Path::new(&common_service.config.log_path).join("app.log");
    match std::fs::read_to_string(&log_file_path) {
        Ok(log_content) => {
            info!("日志读取成功 - 文件大小: {} 字节", log_content.len());
            (StatusCode::OK, Json(serde_json::json!({"logs": log_content})))
        },
        Err(e) => {
            error!("日志读取失败: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": format!("{}", e)})))
        }
    }
}

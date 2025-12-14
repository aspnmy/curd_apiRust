use axum::{Extension, Json, http::StatusCode, extract::Path};
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

/// 处理基于version的API请求
/// 路径结构: /api/{version}/{add、check、update、isdel}
pub async fn handle_file_type_request(
    Extension(common_service): Extension<Arc<CommonService>>,
    Path(params): Path<(String, String)>, // (version, operation)
    Json(mut request): Json<CommonRequest>,
) -> (StatusCode, Json<CommonResponse>) {
    // 从路径参数中提取version和operation
    let (version, operation) = params;
    
    // 记录请求信息
    info!("接收基于version的请求 - version: {}, operation: {}, 服务ID: {}", 
        version, operation, common_service.config.service.id);
    debug!("请求详情: {:?}", request);
    
    // 设置请求的操作类型
    request.operation = operation.clone();    

    
    // 在data中添加version字段，用于记录API版本
    if let serde_json::Value::Object(ref mut obj) = request.data {
        // 添加version字段
        obj.insert("version".to_string(), serde_json::Value::String(version.clone()));
        
        // 处理img2dicom类型的特殊要求
        if let Some(serde_json::Value::String(file_type)) = obj.get("file_type") {
            if file_type == "img2dicom" {
                // 根据img2dicom.rule.md要求，添加必要的字段
                // 这些字段将在服务层或转换方法中被填充
                // image_content: 上传的image文件的base64编码后的内容
                // dicom_path: 转换后的dicom文件的路径
                // dicom_content: dicom文件base64编码后的内容
                
                // 确保这些字段存在，即使它们的值是空的
                if !obj.contains_key("image_content") {
                    obj.insert("image_content".to_string(), serde_json::Value::String("".to_string()));
                }
                
                if !obj.contains_key("dicom_path") {
                    obj.insert("dicom_path".to_string(), serde_json::Value::String("".to_string()));
                }
                
                if !obj.contains_key("dicom_content") {
                    obj.insert("dicom_content".to_string(), serde_json::Value::String("".to_string()));
                }
            }
        }
    }
    
    // 调用通用请求处理函数
    handle_common_request(Extension(common_service), Json(request)).await
}

/// 处理基于file_type的健康检查请求
/// 路径结构: /api/{version}/health
pub async fn handle_file_type_health_check(
    Extension(common_service): Extension<Arc<CommonService>>,
    Path(file_type): Path<String>,
) -> (StatusCode, Json<HealthResponse>) {
    // 记录请求信息
    info!("接收基于file_type的健康检查请求 - file_type: {}, 服务ID: {}", 
        file_type, common_service.config.service.id);
    
    // 调用现有的健康检查方法
    match common_service.get_health().await {
        Ok(health) => {
            // 在健康检查响应中添加file_type信息
            // 注意：HealthResponse结构体没有file_type字段，我们只能返回标准的健康检查响应
            info!("基于file_type的健康检查成功 - file_type: {}, 状态: healthy", file_type);
            (StatusCode::OK, Json(health))
        },
        Err(e) => {
            error!("基于file_type的健康检查失败 - file_type: {}, 错误: {:?}", file_type, e);
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

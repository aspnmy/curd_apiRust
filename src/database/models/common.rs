use serde::{Deserialize, Serialize};

/// 通用请求结构体，用于处理增删改查操作
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommonRequest {
    /// 操作类型：add, check, update, isdel
    pub operation: String,
    /// 表名
    pub table_name: String,
    /// 操作数据
    pub data: serde_json::Value,
    /// 查询条件
    pub where_conditions: Option<Vec<Condition>>,
    /// 查询字段（用于check操作）
    pub fields: Option<Vec<String>>,
    /// 软删除配置
    pub soft_delete_config: Option<SoftDeleteConfig>,
    /// 加密配置，用于媒体数据加密
    pub encryption_config: Option<EncryptionConfig>,
}

/// 条件结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Condition {
    /// 字段名
    pub field: String,
    /// 操作符: =, >, <, >=, <=, !=, LIKE, IN, NOT IN, IS NULL, IS NOT NULL
    pub operator: String,
    /// 值
    pub value: serde_json::Value,
}

/// 软删除配置结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SoftDeleteConfig {
    /// 软删除字段名
    pub field: String,
    /// 软删除值
    pub value: String,
}

/// 加密配置结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncryptionConfig {
    /// 是否需要加密
    pub need_encryption: bool,
    /// 加密字段名
    pub encryption_fields: Vec<String>,
    /// 用户名，用于获取用户密钥
    pub username: String,
    /// UKey部分B，用于加密
    pub ukey_part_b: String,
}

/// 通用响应结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommonResponse {
    /// 操作是否成功
    pub success: bool,
    /// 响应消息
    pub message: String,
    /// 响应数据
    pub data: Option<serde_json::Value>,
    /// 受影响的行数
    pub affected_rows: Option<u64>,
    /// 总记录数（用于check操作）
    pub total: Option<u64>,
    /// 加密信息（如果有加密操作）
    pub encryption_info: Option<serde_json::Value>,
    /// 服务ID，用于跟踪请求处理的服务
    pub service_id: String,
    /// 服务角色，用于跟踪请求处理的服务角色
    pub service_role: String,
}

/// 用于批量操作的请求结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatchRequest {
    /// 批量操作的请求列表
    pub requests: Vec<CommonRequest>,
}

/// 用于批量操作的响应结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatchResponse {
    /// 批量操作的结果列表
    pub results: Vec<CommonResponse>,
    /// 总成功数
    pub success_count: u64,
    /// 总失败数
    pub failure_count: u64,
    /// 服务ID，用于跟踪请求处理的服务
    pub service_id: String,
    /// 服务角色，用于跟踪请求处理的服务角色
    pub service_role: String,
}

/// 健康检查响应结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HealthResponse {
    /// 服务状态
    pub status: String,
    /// 服务ID
    pub service_id: String,
    /// 服务角色
    pub service_role: String,
    /// 服务版本
    pub version: String,
    /// 数据库连接状态
    pub database_status: String,
    /// 服务启动时间
    pub started_at: String,
}

/// 错误响应结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct ErrorResponse {
    /// 错误代码
    pub code: String,
    /// 错误消息
    pub message: String,
    /// 错误详情
    pub details: Option<String>,
    /// 服务ID
    pub service_id: String,
    /// 服务角色
    pub service_role: String,
    /// 时间戳
    pub timestamp: String,
}

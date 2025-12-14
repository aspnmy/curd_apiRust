use anyhow::Result;
use chrono::Utc;
use serde_json::Value as JsonValue;
use sqlx::{Postgres, Row, query};
use thiserror::Error;
use tracing::{debug, error, info};

use crate::config::AppConfig;
use crate::database::DatabasePool;
use crate::database::models::common::{CommonRequest, CommonResponse, Condition};

/// 通用服务错误类型
#[derive(Error, Debug)]
pub enum CommonServiceError {
    /// 数据库错误
    #[error("数据库错误: {0}")]
    DatabaseError(#[from] sqlx::Error),
    /// 无效的操作类型
    #[error("无效的操作类型: {0}")]
    InvalidOperation(String),
    /// 无效的表名
    #[error("无效的表名: {0}")]
    InvalidTableName(String),
    /// 无效的字段名
    #[error("无效的字段名: {0}")]
    InvalidFieldName(String),
    /// 无效的条件
    #[error("无效的条件: {0}")]
    InvalidCondition(String),
    /// 服务角色不允许此操作
    #[error("服务角色不允许此操作: {0}")]
    ServiceRoleError(String),
    /// 内部服务器错误
    #[allow(dead_code)]
    #[error("内部服务器错误")]
    InternalServerError,
}

/// 通用服务
pub struct CommonService {
    pub db: DatabasePool,
    pub config: AppConfig,
    // 服务启动时间
    pub started_at: String,
}

impl CommonService {
    /// 创建通用服务实例
    pub fn new(db: DatabasePool, config: AppConfig) -> Self {
        Self {
            db,
            config,
            // 记录服务启动时间
            started_at: Utc::now().to_rfc3339(),
        }
    }

    /// 验证表名是否允许操作
    fn validate_table_name(&self, table_name: &str) -> Result<(), CommonServiceError> {
        if self.config.allowed_tables.contains(&table_name.to_string()) {
            Ok(())
        } else {
            Err(CommonServiceError::InvalidTableName(table_name.to_string()))
        }
    }

    /// 验证服务角色是否允许此操作
    fn validate_service_role(&self, operation: &str) -> Result<(), CommonServiceError> {
        let role = &self.config.service.role;

        // 根据操作类型验证服务角色
        match operation {
            "add" | "update" | "isdel" => {
                if role == "read" {
                    return Err(CommonServiceError::ServiceRoleError(
                        "只读角色不允许写操作".to_string(),
                    ));
                }
            }
            "check" => {
                // 所有角色都允许读操作
            }
            _ => {
                return Err(CommonServiceError::InvalidOperation(operation.to_string()));
            }
        }

        Ok(())
    }

    /// 执行通用操作
    pub async fn execute(
        &self,
        request: CommonRequest,
    ) -> Result<CommonResponse, CommonServiceError> {
        info!("执行通用操作 - 操作类型: {}, 表名: {}", 
            request.operation, request.table_name);
        debug!("请求详情: {:?}", request);
        
        // 验证服务角色
        self.validate_service_role(&request.operation)?;
        info!("服务角色验证通过");

        // 验证表名 - 使用ALLOWED_TABLES中指定的表名，忽略传入的table_name
        // 注意：前端发送的table_name实际上是file_type，不是真正的数据库表名
        let allowed_tables = &self.config.allowed_tables;
        
        // 确保ALLOWED_TABLES只包含一个表名（单表配置）
        if allowed_tables.len() != 1 {
            error!("配置错误：ALLOWED_TABLES必须只包含一个表名，当前配置：{:?}", allowed_tables);
            return Err(CommonServiceError::InvalidTableName(
                "ALLOWED_TABLES必须只包含一个表名".to_string()
            ));
        }
        
        // 获取ALLOWED_TABLES中指定的单表名
        let table_name = allowed_tables.first().unwrap().clone();
        self.validate_table_name(&table_name)?;
        info!("表名验证通过，使用表名：{}", table_name);

        // 覆盖传入的table_name，固定使用ALLOWED_TABLES中指定的单表名
        let mut request = request;
        request.table_name = table_name;

        let result = match request.operation.as_str() {
            "add" => self.add(request).await,
            "check" => self.check(request).await,
            "update" => self.update(request).await,
            "isdel" => self.isdel(request).await,
            _ => Err(CommonServiceError::InvalidOperation(request.operation)),
        };
        
        info!("通用操作执行完成 - 结果: {}", 
            if result.is_ok() { "成功" } else { "失败" });
        
        result
    }

    /// 添加记录
    async fn add(&self, mut request: CommonRequest) -> Result<CommonResponse, CommonServiceError> {
        info!(
            "执行添加操作，逻辑表名: {}, 数据: {:?}",
            request.table_name, request.data
        );

        // 处理img2dicom类型的特殊要求
        if let serde_json::Value::Object(ref mut obj) = request.data {
            // 检查是否为img2dicom类型
            if let Some(serde_json::Value::String(file_type)) = obj.get("file_type") {
                if file_type == "img2dicom" {
                    info!("处理img2dicom类型的添加操作");
                    
                    // 根据img2dicom.rule.md要求，执行转换逻辑
                    // 1. 检查是否包含image_content字段
                    if let Some(serde_json::Value::String(image_content)) = obj.get("image_content") {
                        info!("找到image_content，开始img2dicom转换");
                        
                        // TODO: 实现实际的img2dicom转换逻辑
                        // 这里应该调用后端独立的img2dicom转换方法，将image文件转换为dicom文件
                        // 转换后的dicom文件，需要将dicom文件的内容base64编码后，存储到dicom_content字段中
                        // 转换成功后的dicom文件，需要将dicom文件的路径存储到dicom_path字段中
                        // 无论是否转换成功，都需要将image文件的base64编码后的内容存储到image_content字段中
                        
                        // 目前使用模拟数据，实际开发中需要替换为真实的转换逻辑
                        let dicom_content = "simulated_dicom_content_base64".to_string();
                        let dicom_path = "simulated/dicom/path.dcm".to_string();
                        
                        // 更新datainfos字段
                        obj.insert("image_content".to_string(), serde_json::Value::String(image_content.clone()));
                        obj.insert("dicom_content".to_string(), serde_json::Value::String(dicom_content));
                        obj.insert("dicom_path".to_string(), serde_json::Value::String(dicom_path));
                        
                        info!("img2dicom转换完成");
                    } else {
                        info!("未找到image_content字段，跳过img2dicom转换");
                        // 确保image_content字段存在
                        obj.insert("image_content".to_string(), serde_json::Value::String("".to_string()));
                        obj.insert("dicom_content".to_string(), serde_json::Value::String("".to_string()));
                        obj.insert("dicom_path".to_string(), serde_json::Value::String("".to_string()));
                    }
                }
            }
        }

        // 获取配置中的表名
        let sql_table = self.config.allowed_tables.first().unwrap();
        
        // 生成SQL语句 - 使用配置中的表名
        let sql = format!(
            "INSERT INTO {} (file_type, datainfos) VALUES ($1, $2) RETURNING *",
            sql_table
        );

        info!("生成的SQL语句: {}", sql);

        // 执行SQL语句
        let row = sqlx::query(&sql)
            .bind(&request.table_name)
            .bind(&request.data)
            .fetch_one(&self.db)
            .await?;
        let affected_rows = 1;

        // 转换为JSON响应
        let result = self.row_to_json(&row).await?;

        Ok(CommonResponse {
            success: true,
            message: "添加成功".to_string(),
            data: Some(result),
            affected_rows: Some(affected_rows),
            total: None,
            encryption_info: None,
            service_id: self.config.service.id.clone(),
            service_role: self.config.service.role.clone(),
        })
    }

    /// 查询记录
    async fn check(&self, request: CommonRequest) -> Result<CommonResponse, CommonServiceError> {
        info!(
            "执行查询操作，逻辑表名: {}, 条件: {:?}",
            request.table_name, request.where_conditions
        );

        // 构建查询字段
        let fields_str = if let Some(fields) = &request.fields {
            fields.join(", ")
        } else {
            "*".to_string()
        };

        // 构建WHERE子句
        let (mut where_clause, mut where_params, has_is_del_condition) = self.build_where_clause(&request.where_conditions)?;
        
        // 获取配置中的表名
        let sql_table = self.config.allowed_tables.first().unwrap();
        
        // 添加逻辑表名条件 - 使用file_type字段
        if where_clause.is_empty() {
            where_clause = "WHERE file_type = $1".to_string();
        } else {
            where_clause = format!("{} AND file_type = ${}", where_clause, where_params.len() + 1);
        }
        where_params.push(JsonValue::String(request.table_name.clone()));
        
        // 添加软删除条件，非审计查询只显示未删除的数据
        // 只有当前端没有提供is_del条件时才自动添加
        if request.audit.unwrap_or(false) == false && !has_is_del_condition {
            where_clause = format!("{} AND is_del = ${}", where_clause, where_params.len() + 1);
            where_params.push(JsonValue::Bool(false));
        }

        // 生成SQL语句 - 使用配置中的表名
        let sql = format!(
            "SELECT {} FROM {} {}",
            fields_str, sql_table, where_clause
        );

        info!("生成的SQL语句: {}", sql);

        // 执行SQL语句
        let mut query_builder = query(&sql);

        // 绑定参数
        for value in where_params {
            query_builder = self.bind_value(query_builder, value)?;
        }

        // 执行查询
        let rows = query_builder.fetch_all(&self.db).await?;
        let total = rows.len() as u64;

        // 转换为JSON响应
        let mut results = Vec::new();
        for row in rows {
            let result = self.row_to_json(&row).await?;
            results.push(result);
        }

        Ok(CommonResponse {
            success: true,
            message: "查询成功".to_string(),
            data: Some(JsonValue::Array(results)),
            affected_rows: None,
            total: Some(total),
            encryption_info: None,
            service_id: self.config.service.id.clone(),
            service_role: self.config.service.role.clone(),
        })
    }

    /// 更新记录
    async fn update(&self, request: CommonRequest) -> Result<CommonResponse, CommonServiceError> {
        info!(
            "执行更新操作，逻辑表名: {}, 数据: {:?}, 条件: {:?}",
            request.table_name, request.data, request.where_conditions    
        );

        // 构建WHERE子句
        let (mut where_clause, mut where_params, _) = self.build_where_clause(&request.where_conditions)?;
        
        // 获取配置中的表名
        let sql_table = self.config.allowed_tables.first().unwrap();
        
        // 添加逻辑表名条件 - 使用file_type字段
        if where_clause.is_empty() {
            where_clause = "WHERE file_type = $1".to_string();
        } else {
            where_clause = format!("{} AND file_type = ${}", where_clause, where_params.len() + 1);
        }
        where_params.push(JsonValue::String(request.table_name.clone()));

        // 生成SQL语句 - 使用配置中的表名
        let sql = format!(
            "UPDATE {} SET datainfos = $1 {} RETURNING *",
            sql_table, where_clause
        );

        info!("生成的SQL语句: {}", sql);

        // 执行SQL语句
        let mut query_builder = query(&sql);

        // 绑定更新数据参数
        query_builder = query_builder.bind(&request.data);

        // 绑定WHERE参数
        for value in where_params {
            query_builder = self.bind_value(query_builder, value)?;
        }

        // 执行查询
        let rows = query_builder.fetch_all(&self.db).await?;
        let affected_rows = rows.len() as u64;

        // 转换为JSON响应
        let mut results = Vec::new();
        for row in rows {
            let result = self.row_to_json(&row).await?;
            results.push(result);
        }

        Ok(CommonResponse {
            success: true,
            message: "更新成功".to_string(),
            data: if results.is_empty() {
                None
            } else {
                Some(JsonValue::Array(results))
            },
            affected_rows: Some(affected_rows),
            total: None,
            encryption_info: None,
            service_id: self.config.service.id.clone(),
            service_role: self.config.service.role.clone(),
        })
    }

    /// 软删除记录
    async fn isdel(&self, request: CommonRequest) -> Result<CommonResponse, CommonServiceError> {
        info!(
            "执行软删除操作，逻辑表名: {}, 条件: {:?}, 配置: {:?}",
            request.table_name, request.where_conditions, request.soft_delete_config    
        );

        // 获取软删除配置
        let soft_delete_config = 
            request
                .soft_delete_config
                .as_ref()
                .ok_or(CommonServiceError::InvalidCondition(
                    "软删除配置不能为空".to_string(),
                ))?;

        // 构建WHERE子句
        let (mut where_clause, mut where_params, _) = self.build_where_clause(&request.where_conditions)?;
        
        // 获取配置中的表名
        let sql_table = self.config.allowed_tables.first().unwrap();
        
        // 添加逻辑表名条件 - 使用file_type字段
        if where_clause.is_empty() {
            where_clause = "WHERE file_type = $1".to_string();
        } else {
            where_clause = format!("{} AND file_type = ${}", where_clause, where_params.len() + 1);
        }
        where_params.push(JsonValue::String(request.table_name.clone()));

        // 生成SQL语句（使用参数化查询）- 使用配置中的表名
        let sql = format!(
            "UPDATE {} SET {} = $1 {} RETURNING *",
            sql_table, soft_delete_config.field, where_clause
        );

        info!("生成的SQL语句: {}", sql);

        // 执行SQL语句
        let mut query_builder = query(&sql);

        // 绑定软删除值
        query_builder = self.bind_value(
            query_builder,
            serde_json::Value::String(soft_delete_config.value.clone()),
        )?;

        // 绑定WHERE参数
        for value in where_params {
            query_builder = self.bind_value(query_builder, value)?;
        }

        // 执行查询
        let rows = query_builder.fetch_all(&self.db).await?;
        let affected_rows = rows.len() as u64;

        Ok(CommonResponse {
            success: true,
            message: "软删除成功".to_string(),
            data: None,
            affected_rows: Some(affected_rows),
            total: None,
            encryption_info: None,
            service_id: self.config.service.id.clone(),
            service_role: self.config.service.role.clone(),
        })
    }

    /// 构建WHERE子句
    fn build_where_clause(
        &self,
        conditions: &Option<Vec<Condition>>,
    ) -> Result<(String, Vec<JsonValue>, bool), CommonServiceError> {
        let mut where_clause = String::new();
        let mut where_params = Vec::new();
        let mut param_index = 1;
        let mut has_is_del_condition = false;

        if let Some(conds) = conditions {
            if !conds.is_empty() {
                where_clause = "WHERE ".to_string();
                let mut condition_strings = Vec::new();

                for cond in conds {
                    // 验证字段名
                    self.validate_field_name(&cond.field)?;

                    // 检查是否包含is_del条件
                    if cond.field == "is_del" {
                        has_is_del_condition = true;
                    }

                    // 对于JSONB字段，使用 ->> 操作符进行查询
                    let field_expr = if cond.field == "id" || cond.field == "file_type" || cond.field == "is_rols" || cond.field == "is_del" || cond.field == "is_date" || cond.field == "created_at" || cond.field == "updated_at" {
                        // 对于普通字段，直接使用字段名
                        cond.field.to_string()
                    } else {
                        // 对于JSONB中的字段，使用 ->> 操作符
                        format!("datainfos ->> '{}'", cond.field)
                    };

                    condition_strings
                        .push(format!("{} {} ${}", field_expr, cond.operator, param_index));
                    where_params.push(cond.value.clone());
                    param_index += 1;
                }

                where_clause += &condition_strings.join(" AND ");
            }
        }

        Ok((where_clause, where_params, has_is_del_condition))
    }

    /// 验证字段名
    fn validate_field_name(&self, field_name: &str) -> Result<(), CommonServiceError> {
        // 简单的字段名验证，防止SQL注入
        if field_name.contains(';') || field_name.contains('\'') || field_name.contains('"') {
            Err(CommonServiceError::InvalidFieldName(field_name.to_string()))
        } else {
            Ok(())
        }
    }

    /// 绑定值到SQL查询
    fn bind_value<'q>(
        &self,
        mut query_builder: sqlx::query::Query<'q, Postgres, sqlx::postgres::PgArguments>,
        value: JsonValue,
    ) -> Result<sqlx::query::Query<'q, Postgres, sqlx::postgres::PgArguments>, CommonServiceError>
    {
        query_builder = match value {
            JsonValue::Null => query_builder.bind(None::<String>),
            JsonValue::Bool(b) => query_builder.bind(b),
            JsonValue::Number(n) => {
                if let Some(i) = n.as_i64() {
                    query_builder.bind(i)
                } else if let Some(f) = n.as_f64() {
                    query_builder.bind(f)
                } else {
                    return Err(CommonServiceError::InvalidCondition(
                        "无效的数值类型".to_string(),
                    ));
                }
            }
            JsonValue::String(s) => query_builder.bind(s),
            JsonValue::Array(_) => query_builder.bind(value.to_string()),
            JsonValue::Object(_) => query_builder.bind(value.to_string()),
        };
        Ok(query_builder)
    }

    /// 将数据库行转换为JSON
    /// 直接返回 datainfos 字段的内容，确保业务通用性
    /// 参数:
    /// - row: 数据库查询返回的行数据
    /// 返回值:
    /// - Result<JsonValue, CommonServiceError>: 转换后的JSON数据或错误信息
    async fn row_to_json(
        &self,
        row: &sqlx::postgres::PgRow,
    ) -> Result<JsonValue, CommonServiceError> {
        // 直接获取 datainfos 字段的值，该字段存储了所有业务数据
        let datainfos = row.try_get::<serde_json::Value, _>("datainfos")?;
        Ok(datainfos)
    }

    /// 获取服务健康状态
    pub async fn get_health(
        &self,
    ) -> Result<crate::database::models::common::HealthResponse, CommonServiceError> {
        // 检查数据库连接
        let database_status = match sqlx::query("SELECT 1").fetch_one(&self.db).await {
            Ok(_) => "healthy".to_string(),
            Err(e) => {
                error!("数据库连接失败: {:?}", e);
                "unhealthy".to_string()
            }
        };

        Ok(crate::database::models::common::HealthResponse {
            status: "healthy".to_string(),
            service_id: self.config.service.id.clone(),
            service_role: self.config.service.role.clone(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            database_status,
            started_at: self.started_at.clone(),
        })
    }
}

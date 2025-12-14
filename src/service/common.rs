use anyhow::Result;
use chrono::Utc;
use serde_json::Value as JsonValue;
use sqlx::{Column, Postgres, Row, query};
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

        // 验证表名
        self.validate_table_name(&request.table_name)?;
        info!("表名验证通过");

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
    async fn add(&self, request: CommonRequest) -> Result<CommonResponse, CommonServiceError> {
        info!(
            "执行添加操作，逻辑表名: {}, 数据: {:?}",
            request.table_name, request.data
        );

        // 生成SQL语句 - 使用通用表结构
        let sql = format!(
            "INSERT INTO common_data (table_name, datainfos) VALUES ($1, $2) RETURNING *"
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
        let (mut where_clause, mut where_params) = self.build_where_clause(&request.where_conditions)?;
        
        // 添加逻辑表名条件
        if where_clause.is_empty() {
            where_clause = "WHERE table_name = $1".to_string();
        } else {
            where_clause = format!("{} AND table_name = ${}", where_clause, where_params.len() + 1);
        }
        where_params.push(JsonValue::String(request.table_name.clone()));
        
        // 添加软删除条件，非审计查询只显示未删除的数据
        if request.audit.unwrap_or(false) == false {
            where_clause = format!("{} AND is_del = ${}", where_clause, where_params.len() + 1);
            where_params.push(JsonValue::Bool(false));
        }

        // 生成SQL语句 - 使用通用表结构
        let sql = format!(
            "SELECT {} FROM common_data {}",
            fields_str, where_clause
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
        let (mut where_clause, mut where_params) = self.build_where_clause(&request.where_conditions)?;
        
        // 添加逻辑表名条件
        if where_clause.is_empty() {
            where_clause = "WHERE table_name = $1".to_string();
        } else {
            where_clause = format!("{} AND table_name = ${}", where_clause, where_params.len() + 1);
        }
        where_params.push(JsonValue::String(request.table_name.clone()));

        // 生成SQL语句 - 使用通用表结构
        let sql = format!(
            "UPDATE common_data SET datainfos = $1 {} RETURNING *",
            where_clause
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
        let (mut where_clause, mut where_params) = self.build_where_clause(&request.where_conditions)?;
        
        // 添加逻辑表名条件
        if where_clause.is_empty() {
            where_clause = "WHERE table_name = $1".to_string();
        } else {
            where_clause = format!("{} AND table_name = ${}", where_clause, where_params.len() + 1);
        }
        where_params.push(JsonValue::String(request.table_name.clone()));

        // 生成SQL语句（使用参数化查询）- 使用通用表结构
        let sql = format!(
            "UPDATE common_data SET {} = $1 {} RETURNING *",
            soft_delete_config.field, where_clause
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
    ) -> Result<(String, Vec<JsonValue>), CommonServiceError> {
        let mut where_clause = String::new();
        let mut where_params = Vec::new();
        let mut param_index = 1;

        if let Some(conds) = conditions {
            if !conds.is_empty() {
                where_clause = "WHERE ".to_string();
                let mut condition_strings = Vec::new();

                for cond in conds {
                    // 验证字段名
                    self.validate_field_name(&cond.field)?;

                    // 对于JSONB字段，使用 ->> 操作符进行查询
                    let field_expr = if cond.field == "id" || cond.field == "table_name" || cond.field == "is_rols" || cond.field == "is_del" || cond.field == "is_date" || cond.field == "created_at" || cond.field == "updated_at" {
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

        Ok((where_clause, where_params))
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
    async fn row_to_json(
        &self,
        row: &sqlx::postgres::PgRow,
    ) -> Result<JsonValue, CommonServiceError> {
        let columns = row.columns();
        let mut result = JsonValue::Object(serde_json::Map::new());

        for column in columns {
            // 使用Column trait的name()方法获取列名
            let column_name = column.name();

            // 使用row.try_get方法获取值，支持泛型转换
            let json_value = match row.try_get::<serde_json::Value, _>(column_name) {
                Ok(val) => val,
                Err(_) => {
                    // 如果无法直接转换为JSON，尝试转换为字符串
                    match row.try_get::<String, _>(column_name) {
                        Ok(s) => JsonValue::String(s),
                        Err(_) => JsonValue::Null,
                    }
                }
            };

            result
                .as_object_mut()
                .unwrap()
                .insert(column_name.to_string(), json_value);
        }

        Ok(result)
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

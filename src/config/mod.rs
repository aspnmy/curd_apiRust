use std::env;
use serde::Deserialize;
use tracing::info;
use anyhow::Result;

/// 应用配置结构体
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    /// 服务器配置
    pub server: ServerConfig,
    /// 数据库配置
    pub database: DatabaseConfig,
    /// JWT配置
    pub jwt: JwtConfig,
    /// 加密配置
    #[allow(dead_code)]
    pub encryption: EncryptionConfig,
    /// 服务角色配置
    pub service: ServiceRoleConfig,
    /// 允许操作的表名白名单
    pub allowed_tables: Vec<String>,
}

/// 服务器配置
#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    /// 服务器地址
    pub host: String,
    /// 服务器端口
    pub port: u16,
    /// 是否启用HTTPS
    #[allow(dead_code)]
    pub https: bool,
}

/// 数据库配置
#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    /// 数据库URL
    pub url: String,
    /// 最大连接数
    pub max_connections: u32,
    /// 最小连接数
    pub min_connections: u32,
}

/// JWT配置
#[derive(Debug, Deserialize, Clone)]
pub struct JwtConfig {
    /// JWT密钥
    pub secret: String,
    /// JWT过期时间（秒）
    #[allow(dead_code)]
    pub expires_in: i64,
    /// JWT刷新时间（秒）
    #[allow(dead_code)]
    pub refresh_in: i64,
}

/// 加密配置
#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct EncryptionConfig {
    /// 加密算法
    pub algorithm: String,
    /// 密钥长度
    pub key_length: u32,
    /// 迭代次数
    pub iterations: u32,
}

/// 服务角色配置
#[derive(Debug, Deserialize, Clone)]
pub struct ServiceRoleConfig {
    /// 服务角色：read, write, mixed
    pub role: String,
    /// 服务ID
    pub id: String,
}

impl AppConfig {
    /// 从环境变量加载配置
    pub fn from_env() -> Result<Self> {
        info!("从环境变量加载配置");
        
        // 从环境变量读取允许的表名，支持逗号分隔的字符串
        let allowed_tables = env::var("ALLOWED_TABLES")
            .unwrap_or("users,resources,encryption_keys".to_string())
            .split(',')
            .map(|table| table.trim().to_string())
            .filter(|table| !table.is_empty())
            .collect();
        
        let config = Self {
            server: ServerConfig {
                host: env::var("SERVER_HOST").unwrap_or("0.0.0.0".to_string()),
                port: env::var("SERVER_PORT").unwrap_or("8000".to_string()).parse()?,
                https: env::var("HTTPS").unwrap_or("false".to_string()).parse()?,
            },
            database: DatabaseConfig {
                url: env::var("DATABASE_URL").unwrap_or("postgres://user:password@localhost:5432/secret_gallery".to_string()),
                max_connections: env::var("DATABASE_MAX_CONNECTIONS").unwrap_or("10".to_string()).parse()?,
                min_connections: env::var("DATABASE_MIN_CONNECTIONS").unwrap_or("2".to_string()).parse()?,
            },
            jwt: JwtConfig {
                secret: env::var("JWT_SECRET").unwrap_or("your_secret_key".to_string()),
                expires_in: env::var("JWT_EXPIRES_IN").unwrap_or("3600".to_string()).parse()?,
                refresh_in: env::var("JWT_REFRESH_IN").unwrap_or("86400".to_string()).parse()?,
            },
            encryption: EncryptionConfig {
                algorithm: env::var("ENCRYPTION_ALGORITHM").unwrap_or("aes-256-gcm".to_string()),
                key_length: env::var("ENCRYPTION_KEY_LENGTH").unwrap_or("32".to_string()).parse()?,
                iterations: env::var("ENCRYPTION_ITERATIONS").unwrap_or("100000".to_string()).parse()?,
            },
            service: ServiceRoleConfig {
                role: env::var("SERVICE_ROLE").unwrap_or("mixed".to_string()),
                id: env::var("SERVICE_ID").unwrap_or("crud-01".to_string()),
            },
            allowed_tables,
        };
        
        Ok(config)
    }
    
    /// 验证配置
    pub fn validate(&self) -> Result<()> {
        info!("验证配置");
        
        // 验证服务角色
        let valid_roles = vec!["read", "write", "mixed"];
        if !valid_roles.contains(&self.service.role.as_str()) {
            anyhow::bail!("无效的服务角色: {}", self.service.role);
        }
        
        // 验证JWT密钥长度
        if self.jwt.secret.len() < 16 {
            anyhow::bail!("JWT密钥长度至少为16个字符");
        }
        
        // 验证数据库URL
        if self.database.url.is_empty() {
            anyhow::bail!("数据库URL不能为空");
        }
        
        info!("配置验证通过");
        Ok(())
    }
}
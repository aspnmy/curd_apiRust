use anyhow::Result;
use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};
use tracing::info;

/// 插件配置结构体，用于从环境变量或配置文件中读取插件配置
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RuleConfig {
    /// 是否启用插件系统
    pub isrule: bool,
    /// 启用的插件列表
    pub enabled_plugins: Vec<String>,
}

/// 插件trait定义，所有插件都需要实现这个trait
pub trait RulePlugin {
    /// 获取插件名称
    fn name(&self) -> &'static str;
    
    /// 初始化插件
    fn init(&mut self) -> Result<()> {
        Ok(())
    }
    
    /// 执行插件逻辑
    /// 参数：
    /// - file_type: 文件类型
    /// - data: 输入数据
    /// 返回值：
    /// - Result<serde_json::Value>: 处理结果或错误信息
    fn execute(
        &self,
        file_type: &str,
        data: serde_json::Value,
    ) -> BoxFuture<'_, Result<serde_json::Value>>;
}

/// 插件管理器，用于管理所有插件
pub struct RuleManager {
    /// 插件配置
    pub config: RuleConfig,
    /// 注册的插件列表
    pub plugins: Vec<Box<dyn RulePlugin + Send + Sync>>,
}

impl RuleManager {
    /// 创建插件管理器
    pub fn new(config: RuleConfig) -> Self {
        Self {
            config,
            plugins: Vec::new(),
            
        }
    }
    
    /// 注册插件
    pub fn register_plugin(&mut self, plugin: impl RulePlugin + Send + Sync + 'static) {
        let plugin_name = plugin.name().to_string();
        self.plugins.push(Box::new(plugin));
        info!("注册插件: {}", plugin_name);
    }
    
    /// 初始化所有插件
    pub fn init(&mut self) -> Result<()> {
        if !self.config.isrule {
            info!("插件系统未启用");
            return Ok(());
        }
        
        info!("初始化插件系统，启用的插件: {:?}", self.config.enabled_plugins);
        
        for plugin in &mut self.plugins {
            if self.config.enabled_plugins.contains(&plugin.name().to_string()) {
                plugin.init()?;
                info!("初始化插件成功: {}", plugin.name());
            } else {
                info!("跳过插件: {}", plugin.name());
            }
        }
        
        Ok(())
    }
    
    /// 执行插件
    pub async fn execute(
        &self,
        file_type: &str,
        data: serde_json::Value,
    ) -> Result<serde_json::Value> {
        if !self.config.isrule {
            // 插件系统未启用，直接返回原始数据
            return Ok(data);
        }
        
        // 遍历所有启用的插件，执行插件逻辑
        let mut result = data;
        for plugin in &self.plugins {
            if self.config.enabled_plugins.contains(&plugin.name().to_string()) {
                result = plugin.execute(file_type, result).await?;
                info!("执行插件成功: {}", plugin.name());
            }
        }
        
        Ok(result)
    }
}

/// 从环境变量或配置文件中读取插件配置
pub fn load_rule_config() -> Result<RuleConfig> {
    // 从环境变量中读取配置，支持Dockerfile参数配置
    let isrule = std::env::var("ISRULE")
        .unwrap_or("false".to_string())
        .parse::<bool>()?;
    
    let enabled_plugins = std::env::var("ENABLED_PLUGINS")
        .unwrap_or("image2base64,image2dicom".to_string())
        .split(",")
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    
    Ok(RuleConfig {
        isrule,
        enabled_plugins,
    })
}

// 导出子模块
pub mod image2base64;
pub mod image2dicom;

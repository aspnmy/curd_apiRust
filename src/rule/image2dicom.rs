use anyhow::{anyhow, Result};
use base64::{engine::general_purpose::STANDARD as BASE64_STD,
             Engine as _};
use chrono::Utc;
use futures::future::BoxFuture;
use rand::random;
use reqwest::Client;
use serde_json::Value as JsonValue;
use tracing::info;

use super::RulePlugin;

/// 图片转DICOM插件
pub struct Image2DicomPlugin {
    // 插件配置，可以根据需要添加配置项
    /// DICOM转换服务地址
    dicom_convert_service: String,
    /// HTTP客户端
    client: Client,
}

impl Image2DicomPlugin {
    /// 创建Image2Dicom插件
    pub fn new() -> Self {
        Self {
            dicom_convert_service: "http://localhost:8080/convert".to_string(),
            client: Client::new(),
        }
    }
    
    /// 执行图片转DICOM转换
    /// 使用reqwest调用外部DICOM转换服务
    async fn convert_image_to_dicom(&self, image_data: &[u8], file_name: &str) -> Result<(Vec<u8>, String)> {
        info!("执行图片转DICOM转换，文件名: {}, 调用服务: {}", file_name, self.dicom_convert_service);
        
        // 调用外部DICOM转换服务
        let response = self.client
            .post(&self.dicom_convert_service)
            .body(image_data.to_vec())
            .header("Content-Type", "image/jpeg")
            .header("X-File-Name", file_name)
            .send()
            .await?;
        
        // 检查响应状态
        if !response.status().is_success() {
            return Err(anyhow!("DICOM转换服务返回错误: {} - {}", 
                response.status(), 
                response.text().await?));
        }
        
        // 获取转换后的DICOM数据
        let dicom_data = response.bytes().await?.to_vec();
        
        // 生成DICOM文件路径
        let dicom_path = format!("/tmp/{}.dcm", file_name.replace('.', "_"));
        
        info!("图片转DICOM转换成功，文件名: {}, DICOM数据大小: {} bytes", file_name, dicom_data.len());
        
        Ok((dicom_data, dicom_path))
    }
    
    /// 从JSON数据中提取图片内容
    fn extract_image_content(&self, data: &JsonValue) -> Result<Vec<u8>> {
        // 尝试从image_content字段中提取
        if let Some(image_content) = data.get("image_content") {
            if let Some(str_content) = image_content.as_str() {
                if str_content.starts_with("data:image") {
                    // 这是一个完整的Base64图片URL，需要提取实际的Base64内容
                    let base64_part = str_content.split(",").nth(1).ok_or(anyhow!("无效的Base64 URL格式"))?;
                    return Ok(BASE64_STD.decode(base64_part)?);
                } else {
                    // 直接是Base64编码
                    return Ok(BASE64_STD.decode(str_content)?);
                }
            }
        }
        
        // 尝试从file_content字段中提取
        if let Some(file_content) = data.get("file_content") {
            if let Some(str_content) = file_content.as_str() {
                if str_content.starts_with("data:image") {
                    let base64_part = str_content.split(",").nth(1).ok_or(anyhow!("无效的Base64 URL格式"))?;
                    return Ok(BASE64_STD.decode(base64_part)?);
                } else {
                    return Ok(BASE64_STD.decode(str_content)?);
                }
            }
        }
        
        Err(anyhow!("未找到有效的图片内容字段"))
    }
}

impl RulePlugin for Image2DicomPlugin {
    /// 获取插件名称
    fn name(&self) -> &'static str {
        "image2dicom"
    }
    
    /// 初始化插件
    fn init(&mut self) -> Result<()> {
        info!("初始化Image2Dicom插件");
        // 可以在这里初始化转换服务连接等
        Ok(())
    }
    
    /// 执行插件逻辑
    fn execute(
        &self,
        file_type: &str,
        data: JsonValue,
    ) -> BoxFuture<'_, Result<JsonValue>> {
        let file_type = file_type.to_string();
        let data = data.clone();
        
        Box::pin(async move {
            info!("执行Image2Dicom插件，文件类型: {}", file_type);
            
            // 只有img2dicom类型才执行转换
            if file_type != "img2dicom" {
                info!("文件类型: {} 不需要执行Image2Dicom转换", file_type);
                return Ok(data);
            }
            
            // 从数据中提取必要字段
            let file_name = data.get("file_name")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown_image");
            
            // 提取图片内容
            let image_data = self.extract_image_content(&data)?;
            
            // 执行图片转DICOM转换
            let (dicom_data, dicom_path) = self.convert_image_to_dicom(&image_data, file_name).await?;
            
            // 将DICOM数据转换为Base64编码
            let dicom_content_base64 = BASE64_STD.encode(&dicom_data);
            
            // 构建完整的Base64 DICOM URL
            let full_dicom_url = format!("data:application/dicom;base64,{}", dicom_content_base64);
            
            // 更新结果
            let mut result = data.clone();
            let mut result_obj = result.as_object_mut().unwrap().clone();
            
            // 保留所有入参字段，并添加新字段
            result_obj.insert("dicom_path".to_string(), JsonValue::String(dicom_path));
            result_obj.insert("dicom_content".to_string(), JsonValue::String(full_dicom_url.clone()));
            result_obj.insert("file_type".to_string(), JsonValue::String("dicom".to_string()));
            result_obj.insert("file_content".to_string(), JsonValue::String(full_dicom_url));
            
            // 如果没有file_id，生成一个
            if !result_obj.contains_key("file_id") {
                let file_id = format!("dicom_{}_{}", 
                    Utc::now().timestamp(),
                    random::<u32>()
                );
                result_obj.insert("file_id".to_string(), JsonValue::String(file_id));
            }
            
            info!("图片转DICOM转换完成，文件名: {}", file_name);
            Ok(JsonValue::Object(result_obj))
        })
    }
}

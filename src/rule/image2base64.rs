use anyhow::Result;
use base64::{engine::general_purpose::STANDARD as BASE64_STD,
             Engine as _};
use futures::future::BoxFuture;
use serde_json::Value as JsonValue;
use tracing::info;

use super::RulePlugin;

/// 图片转Base64插件
pub struct Image2Base64Plugin {
    // 插件配置，可以根据需要添加配置项
}

impl Image2Base64Plugin {
    /// 创建Image2Base64插件
    pub fn new() -> Self {
        Self {}
    }
    
    /// 将图片数据转换为Base64编码
    fn convert_image_to_base64(&self, image_data: &[u8]) -> Result<String> {
        Ok(BASE64_STD.encode(image_data))
    }
    
    /// 从JSON数据中提取图片内容
    fn extract_image_content(&self, data: &JsonValue) -> Result<(Vec<u8>, Option<String>)> {
        // 尝试从不同字段中提取图片内容
        if let Some(image_content) = data.get("image_content") {
            if let Some(str_content) = image_content.as_str() {
                // 如果已经是完整的Base64图片URL，直接返回
                if str_content.starts_with("data:image") {
                    return Ok((vec![], Some(str_content.to_string())));
                } else {
                    // 尝试解码Base64内容
                    return Ok((BASE64_STD.decode(str_content)?, None));
                }
            } else if let Some(bin_content) = image_content.as_array() {
                // 如果是字节数组，直接转换
                return Ok((bin_content.iter()
                    .filter_map(|v| v.as_u64().map(|u| u as u8))
                    .collect(), None));
            }
        }
        
        // 尝试从file_content字段中提取
        if let Some(file_content) = data.get("file_content") {
            if let Some(str_content) = file_content.as_str() {
                if str_content.starts_with("data:image") {
                    return Ok((vec![], Some(str_content.to_string())));
                } else {
                    return Ok((BASE64_STD.decode(str_content)?, None));
                }
            }
        }
        
        Err(anyhow::anyhow!("未找到有效的图片内容字段"))
    }
}

impl RulePlugin for Image2Base64Plugin {
    /// 获取插件名称
    fn name(&self) -> &'static str {
        "image2base64"
    }
    
    /// 初始化插件
    fn init(&mut self) -> Result<()> {
        info!("初始化Image2Base64插件");
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
            info!("执行Image2Base64插件，文件类型: {}", file_type);
            
            // 只有特定文件类型才执行转换
            if !file_type.contains("image") && file_type != "img2dicom" {
                info!("文件类型: {} 不需要执行Image2Base64转换", file_type);
                return Ok(data);
            }
            
            // 尝试提取图片内容
            match self.extract_image_content(&data) {
                Ok((image_data, existing_base64)) => {
                    let full_base64_url = if let Some(base64_url) = existing_base64 {
                        // 如果已经是完整的Base64图片URL，直接使用
                        base64_url
                    } else {
                        // 将图片转换为Base64编码
                        let base64_str = self.convert_image_to_base64(&image_data)?;
                        
                        // 确定图片MIME类型
                        let mime_type = self.detect_mime_type(&image_data).unwrap_or("image/jpeg".to_string());
                        format!("data:{};base64,{}", mime_type, base64_str)
                    };
                    
                    // 构建完整的Base64图片URL
                    let mut result = data.clone();
                    let mut result_obj = result.as_object_mut().unwrap().clone();
                    
                    // 更新结果中的图片字段
                    result_obj.insert("file_content".to_string(), JsonValue::String(full_base64_url.clone()));
                    result_obj.insert("image_content".to_string(), JsonValue::String(full_base64_url));
                    
                    Ok(JsonValue::Object(result_obj))
                },
                Err(e) => {
                    // 如果提取图片内容失败，返回原始数据
                    info!("提取图片内容失败: {}, 返回原始数据", e);
                    Ok(data)
                }
            }
        })
    }
}

impl Image2Base64Plugin {
    /// 简单的MIME类型检测
    fn detect_mime_type(&self, data: &[u8]) -> Option<String> {
        // 检测常见图片格式的魔术数字
        if data.len() < 4 {
            return None;
        }
        
        // JPEG
        if data[0] == 0xFF && data[1] == 0xD8 && data[2] == 0xFF {
            return Some("image/jpeg".to_string());
        }
        
        // PNG
        if &data[0..4] == b"\x89PNG" {
            return Some("image/png".to_string());
        }
        
        // GIF
        if &data[0..3] == b"GIF" {
            return Some("image/gif".to_string());
        }
        
        // WEBP
        if data.len() >= 12 && &data[0..4] == b"RIFF" && &data[8..12] == b"WEBP" {
            return Some("image/webp".to_string());
        }
        
        None
    }
}

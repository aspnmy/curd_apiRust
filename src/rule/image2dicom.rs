use anyhow::{anyhow, Result};
use base64::{engine::general_purpose::STANDARD as BASE64_STD,
             Engine as _};
use chrono::Utc;
use futures::future::BoxFuture;
use rand::random;
use serde_json::Value as JsonValue;
use tracing::info;

use super::RulePlugin;

/// 图片转DICOM插件
pub struct Image2DicomPlugin {
    // 插件配置，可以根据需要添加配置项
    /// DICOM文件保存路径
    dicom_save_path: String,
}

impl Image2DicomPlugin {
    /// 创建Image2Dicom插件
    pub fn new() -> Self {
        // 从环境变量中读取DICOM保存路径
        let dicom_save_path = std::env::var("IMAGE2DICOM_PATH")
            .unwrap_or_default();
        
        // 确定最终的DICOM保存路径
        let dicom_save_path = if dicom_save_path.is_empty() {
            // 环境变量未设置，使用默认路径
            // 检查是否为Windows环境
            if cfg!(target_os = "windows") {
                "./image2dicom".to_string()
            } else {
                "/app/image2dicom".to_string()
            }
        } else {
            // 环境变量已设置，检查是否为Windows环境
            if cfg!(target_os = "windows") {
                // 在Windows环境下，将Linux风格的路径转换为Windows风格
                // 或者使用相对路径
                "./image2dicom".to_string()
            } else {
                // 在非Windows环境下，使用环境变量中的路径
                dicom_save_path
            }
        };
        
        Self {
            dicom_save_path,
        }
    }
    
    /// 执行图片转DICOM转换
    /// 内部实现，不依赖外部服务
    async fn convert_image_to_dicom(&self, image_data: &[u8], file_id: &str, file_name: &str) -> Result<(Vec<u8>, String)> {
        info!("执行图片转DICOM转换，文件名: {}, 文件ID: {}", file_name, file_id);
        
        // 内部实现：生成DICOM格式的数据
        // 这里使用简单的模拟DICOM数据，实际项目中可以使用Rust DICOM库生成真实DICOM文件
        let dicom_data = self.generate_dicom_data(image_data, file_id, file_name)?;
        
        // 生成DICOM文件路径：{dicom_save_path}/{file_id}_{file_name}.dcm
        let dicom_file_name = format!("{}_{}.dcm", file_id, file_name.replace('.', "_"));
        let dicom_path = format!("{}/{}", self.dicom_save_path, dicom_file_name);
        
        // 确保DICOM保存目录存在
        std::fs::create_dir_all(&self.dicom_save_path)?;
        
        // 将DICOM数据写入文件
        std::fs::write(&dicom_path, &dicom_data)?;
        
        info!("图片转DICOM转换成功，文件名: {}, 文件ID: {}, DICOM数据大小: {} bytes, 保存路径: {}", file_name, file_id, dicom_data.len(), dicom_path);
        
        Ok((dicom_data, dicom_path))
    }
    
    /// 生成DICOM格式的数据
    /// 内部实现，不依赖外部服务
    fn generate_dicom_data(&self, image_data: &[u8], file_id: &str, file_name: &str) -> Result<Vec<u8>> {
        // 简单的DICOM文件头模拟
        // 实际项目中应该使用Rust DICOM库（如dicom-rs）生成真实DICOM文件
        let study_uid = format!("STUDY_{}", file_id);
        let series_uid = format!("SERIES_{}", file_id);
        let sop_uid = format!("INSTANCE_{}", file_id);
        let content_date = Utc::now().format("%Y%m%d");
        let content_time = Utc::now().format("%H%M%S");
        
        // 使用单一格式字符串，避免注释和+号连接
        let dicom_header = format!(
            "DICM\nTransferSyntaxUID\n1.2.840.10008.1.2.1\nPatientName\n{}\nStudyInstanceUID\n{}\nSeriesInstanceUID\n{}\nSOPInstanceUID\n{}\nSOPClassUID\n1.2.840.10008.5.1.4.1.1.7\nContentDate\n{}\nContentTime\n{}\nInstanceNumber\n1\nRows\n1024\nColumns\n1024\nBitsAllocated\n8\nBitsStored\n8\nHighBit\n7\nPixelRepresentation\n0\nPhotometricInterpretation\nMONOCHROME2\nSamplesPerPixel\n1\nPlanarConfiguration\n0\nPixelData\n",
            file_name, study_uid, series_uid, sop_uid, content_date, content_time
        );
        
        // 构建完整的DICOM数据
        let mut dicom_data = Vec::new();
        
        // 添加DICOM文件头
        dicom_data.extend_from_slice(dicom_header.as_bytes());
        
        // 添加像素数据（简单的灰度渐变，实际项目中应该使用真实图片数据）
        // 这里使用image_data的前1024*1024字节作为示例
        let pixel_data_len = std::cmp::min(image_data.len(), 1024 * 1024);
        dicom_data.extend_from_slice(&image_data[0..pixel_data_len]);
        
        // 如果像素数据不足，填充0
        let expected_len = 1024 * 1024;
        if pixel_data_len < expected_len {
            dicom_data.extend_from_slice(&vec![0; expected_len - pixel_data_len]);
        }
        
        Ok(dicom_data)
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
            
            // 从数据中提取file_id
            let file_id = data.get("file_id")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown_id");
            
            // 提取图片内容
            let image_data = self.extract_image_content(&data)?;
            
            // 执行图片转DICOM转换
            let (dicom_data, dicom_path) = self.convert_image_to_dicom(&image_data, file_id, file_name).await?;
            
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

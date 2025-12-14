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
        let mut dicom_data = Vec::new();
        
        // 1. DICOM文件格式：128字节preamble + "DICM" + 数据元素
        let preamble = vec![0u8; 128];
        dicom_data.extend_from_slice(&preamble);
        dicom_data.extend_from_slice(b"DICM");
        
        // 2. 添加文件元信息
        // 2.1 SOP Class UID (0008,0016)
        self.write_dicom_element(&mut dicom_data, 0x0008, 0x0016, "UI", "1.2.840.10008.5.1.4.1.1.7")?; // Secondary Capture Image Storage
        
        // 2.2 SOP Instance UID (0008,0018)
        let sop_instance_uid = format!("1.2.840.10008.1.{}.{}.{}", 
            Utc::now().timestamp(), 
            random::<u32>(), 
            random::<u32>());
        self.write_dicom_element(&mut dicom_data, 0x0008, 0x0018, "UI", &sop_instance_uid)?;
        
        // 2.3 Transfer Syntax UID (0002,0010)
        let transfer_syntax_uid = "1.2.840.10008.1.2";
        self.write_dicom_element(&mut dicom_data, 0x0002, 0x0010, "UI", transfer_syntax_uid)?;
        
        // 2.4 Patient Name (0010,0010)
        self.write_dicom_element(&mut dicom_data, 0x0010, 0x0010, "PN", file_name)?;
        
        // 2.5 Patient ID (0010,0020)
        self.write_dicom_element(&mut dicom_data, 0x0010, 0x0020, "LO", file_id)?;
        
        // 2.6 Study Instance UID (0020,000D)
        let study_instance_uid = format!("1.2.840.10008.1.{}.{}", Utc::now().timestamp(), random::<u32>());
        self.write_dicom_element(&mut dicom_data, 0x0020, 0x000D, "UI", &study_instance_uid)?;
        
        // 2.7 Series Instance UID (0020,000E)
        let series_instance_uid = format!("1.2.840.10008.1.{}.{}.{}", Utc::now().timestamp(), random::<u32>(), random::<u32>());
        self.write_dicom_element(&mut dicom_data, 0x0020, 0x000E, "UI", &series_instance_uid)?;
        
        // 2.8 Content Date (0008,0023)
        let content_date = Utc::now().format("%Y%m%d").to_string();
        self.write_dicom_element(&mut dicom_data, 0x0008, 0x0023, "DA", &content_date)?;
        
        // 2.9 Content Time (0008,0033)
        let content_time = Utc::now().format("%H%M%S").to_string();
        self.write_dicom_element(&mut dicom_data, 0x0008, 0x0033, "TM", &content_time)?;
        
        // 2.10 Modality (0008,0060)
        self.write_dicom_element(&mut dicom_data, 0x0008, 0x0060, "CS", "OT")?; // Other
        
        // 2.11 Instance Number (0020,0013)
        self.write_dicom_element(&mut dicom_data, 0x0020, 0x0013, "IS", "1")?;
        
        // 3. 添加图像像素信息
        // 3.1 Photometric Interpretation (0028,0004)
        self.write_dicom_element(&mut dicom_data, 0x0028, 0x0004, "CS", "MONOCHROME2")?;
        
        // 3.2 Samples Per Pixel (0028,0002)
        let samples_per_pixel = 1u16.to_le_bytes();
        self.write_dicom_element(&mut dicom_data, 0x0028, 0x0002, "US", &samples_per_pixel)?;
        
        // 3.3 Rows (0028,0010) - 假设图像为512x512
        let rows = 512u16.to_le_bytes();
        self.write_dicom_element(&mut dicom_data, 0x0028, 0x0010, "US", &rows)?;
        
        // 3.4 Columns (0028,0011) - 假设图像为512x512
        let columns = 512u16.to_le_bytes();
        self.write_dicom_element(&mut dicom_data, 0x0028, 0x0011, "US", &columns)?;
        
        // 3.5 Bits Allocated (0028,0100)
        let bits_allocated = 8u16.to_le_bytes();
        self.write_dicom_element(&mut dicom_data, 0x0028, 0x0100, "US", &bits_allocated)?;
        
        // 3.6 Bits Stored (0028,0101)
        let bits_stored = 8u16.to_le_bytes();
        self.write_dicom_element(&mut dicom_data, 0x0028, 0x0101, "US", &bits_stored)?;
        
        // 3.7 High Bit (0028,0102)
        let high_bit = 7u16.to_le_bytes();
        self.write_dicom_element(&mut dicom_data, 0x0028, 0x0102, "US", &high_bit)?;
        
        // 3.8 Pixel Representation (0028,0103)
        let pixel_representation = 0u16.to_le_bytes();
        self.write_dicom_element(&mut dicom_data, 0x0028, 0x0103, "US", &pixel_representation)?; // Unsigned
        
        // 3.9 Planar Configuration (0028,0006)
        let planar_configuration = 0u16.to_le_bytes();
        self.write_dicom_element(&mut dicom_data, 0x0028, 0x0006, "US", &planar_configuration)?;
        
        // 4. 添加像素数据
        // 创建512x512的灰度图像数据
        let mut pixel_data = Vec::new();
        let target_size = 512 * 512;
        
        if image_data.len() >= target_size {
            pixel_data.extend_from_slice(&image_data[0..target_size]);
        } else {
            pixel_data.extend_from_slice(image_data);
            pixel_data.resize(target_size, 0);
        }
        
        // 写入Pixel Data元素 (7FE0,0010)
        self.write_dicom_element(&mut dicom_data, 0x7FE0, 0x0010, "OW", &pixel_data)?;
        
        Ok(dicom_data)
    }
    
    /// 写入DICOM元素到向量中
    fn write_dicom_element<T: AsRef<[u8]>>(&self, data: &mut Vec<u8>, group: u16, element: u16, vr: &str, value: T) -> Result<()> {
        let value_bytes = value.as_ref();
        
        // 1. 写入组号和元素号（小端字节序）
        data.extend_from_slice(&group.to_le_bytes());
        data.extend_from_slice(&element.to_le_bytes());
        
        // 2. 写入VR
        if vr.len() != 2 {
            return Err(anyhow!("无效的VR: {}", vr));
        }
        data.extend_from_slice(vr.as_bytes());
        
        // 3. 写入保留字节
        data.extend_from_slice(&[0, 0]);
        
        // 4. 写入值长度（小端字节序）
        let value_len = value_bytes.len() as u32;
        data.extend_from_slice(&value_len.to_le_bytes());
        
        // 5. 写入值数据
        data.extend_from_slice(value_bytes);
        
        // 6. 如果值长度为奇数，添加填充字节
        if value_len % 2 != 0 {
            data.push(0);
        }
        
        Ok(())
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

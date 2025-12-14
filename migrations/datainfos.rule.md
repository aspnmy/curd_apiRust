## datainfos 必须结构
- 必须是JSON格式
- 必须包含file_id字段，代表上传的文件的唯一标识符，格式为file_{file_sha256数值的前16位}_{随机数}，如file_08986d298ce8030c_7839
- 必须包含file_name字段，表示上传的文件的文件名
- 必须包含file_type字段，表示上传的文件的类型(可以使用的 file_type 有 image 、dicom、avi、mp3、mp4、img2dicom等)
- 必须包含file_sha256字段，表示上传的文件的SHA256哈希值
- 必须包含file_description字段，表示上传的文件的描述
- 必须包含file_upload_time字段，表示上传的文件的上传时间（UTF-8格式）
- 必须包含file_upload_user字段，表示上传的文件的上传用户
- 必须包含file_upload_ip字段，表示上传的文件的上传IP地址
- 必须包含file_roles字段，表示上传的文件的角色所属
- 必须包含file_status字段，表示上传的文件的状态（如active、inactive等）
- 必须包含file_content字段，表示上传的文件的内容（二进制数据）


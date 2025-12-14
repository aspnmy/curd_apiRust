-- 删除添加的索引
DROP INDEX IF EXISTS idx_common_data_table_name_is_del;
DROP INDEX IF EXISTS idx_common_data_table_name_created_at;

-- 删除部分GIN索引
DROP INDEX IF EXISTS idx_common_data_datainfos_image;
DROP INDEX IF EXISTS idx_common_data_datainfos_dicom;
DROP INDEX IF EXISTS idx_common_data_datainfos_mp3;
DROP INDEX IF EXISTS idx_common_data_datainfos_mp4;
DROP INDEX IF EXISTS idx_common_data_datainfos_img2dicom;

-- 删除针对datainfos中常用字段的索引
DROP INDEX IF EXISTS idx_common_data_datainfos_file_id;
DROP INDEX IF EXISTS idx_common_data_datainfos_file_type;

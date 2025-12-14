-- 添加复合索引：table_name + is_del
CREATE INDEX IF NOT EXISTS idx_common_data_table_name_is_del ON common_data(table_name, is_del);

-- 添加复合索引：table_name + created_at
CREATE INDEX IF NOT EXISTS idx_common_data_table_name_created_at ON common_data(table_name, created_at DESC);

-- 添加针对常用file_type的部分GIN索引
CREATE INDEX IF NOT EXISTS idx_common_data_datainfos_image ON common_data USING GIN(datainfos) WHERE table_name = 'image';
CREATE INDEX IF NOT EXISTS idx_common_data_datainfos_dicom ON common_data USING GIN(datainfos) WHERE table_name = 'dicom';
CREATE INDEX IF NOT EXISTS idx_common_data_datainfos_mp3 ON common_data USING GIN(datainfos) WHERE table_name = 'mp3';
CREATE INDEX IF NOT EXISTS idx_common_data_datainfos_mp4 ON common_data USING GIN(datainfos) WHERE table_name = 'mp4';
CREATE INDEX IF NOT EXISTS idx_common_data_datainfos_img2dicom ON common_data USING GIN(datainfos) WHERE table_name = 'img2dicom';

-- 添加针对datainfos中常用字段的索引
CREATE INDEX IF NOT EXISTS idx_common_data_datainfos_file_id ON common_data((datainfos ->> 'file_id'));
CREATE INDEX IF NOT EXISTS idx_common_data_datainfos_file_type ON common_data((datainfos ->> 'file_type'));

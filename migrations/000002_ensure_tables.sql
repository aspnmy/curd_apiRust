-- 确保数据库表结构完整性的迁移脚本

-- 确保users表的列和约束正确
DO $$
BEGIN
    -- 确保username列存在
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'users' AND column_name = 'username') THEN
        ALTER TABLE users ADD COLUMN username TEXT NOT NULL UNIQUE;
    END IF;
    
    -- 确保hashed_password列存在
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'users' AND column_name = 'hashed_password') THEN
        ALTER TABLE users ADD COLUMN hashed_password TEXT NOT NULL;
    END IF;
    
    -- 确保is_admin列存在且有默认值
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'users' AND column_name = 'is_admin') THEN
        ALTER TABLE users ADD COLUMN is_admin BOOLEAN NOT NULL DEFAULT FALSE;
    END IF;
    
    -- 确保created_at列存在且有默认值
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'users' AND column_name = 'created_at') THEN
        ALTER TABLE users ADD COLUMN created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP;
    END IF;
    
    -- 确保updated_at列存在且有默认值
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'users' AND column_name = 'updated_at') THEN
        ALTER TABLE users ADD COLUMN updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP;
    END IF;
    
    -- 确保key_part_a列存在
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'users' AND column_name = 'key_part_a') THEN
        ALTER TABLE users ADD COLUMN key_part_a TEXT;
    END IF;
    
    -- 确保key_part_b列存在
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'users' AND column_name = 'key_part_b') THEN
        ALTER TABLE users ADD COLUMN key_part_b TEXT;
    END IF;
    
    -- 确保username索引存在
    IF NOT EXISTS (SELECT 1 FROM pg_indexes WHERE tablename = 'users' AND indexname = 'idx_users_username') THEN
        CREATE INDEX idx_users_username ON users(username);
    END IF;
END
$$;

-- 确保resources表的列和约束正确
DO $$
BEGIN
    -- 确保title列存在
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'resources' AND column_name = 'title') THEN
        ALTER TABLE resources ADD COLUMN title TEXT NOT NULL;
    END IF;
    
    -- 确保title_en列存在
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'resources' AND column_name = 'title_en') THEN
        ALTER TABLE resources ADD COLUMN title_en TEXT;
    END IF;
    
    -- 确保description列存在
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'resources' AND column_name = 'description') THEN
        ALTER TABLE resources ADD COLUMN description TEXT;
    END IF;
    
    -- 确保resource_type列存在
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'resources' AND column_name = 'resource_type') THEN
        ALTER TABLE resources ADD COLUMN resource_type TEXT NOT NULL;
    END IF;
    
    -- 确保media_data列存在
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'resources' AND column_name = 'media_data') THEN
        ALTER TABLE resources ADD COLUMN media_data BYTEA NOT NULL;
    END IF;
    
    -- 确保media_type列存在
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'resources' AND column_name = 'media_type') THEN
        ALTER TABLE resources ADD COLUMN media_type TEXT NOT NULL;
    END IF;
    
    -- 确保is_local列存在
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'resources' AND column_name = 'is_local') THEN
        ALTER TABLE resources ADD COLUMN is_local BOOLEAN NOT NULL;
    END IF;
    
    -- 确保encryption_info列存在
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'resources' AND column_name = 'encryption_info') THEN
        ALTER TABLE resources ADD COLUMN encryption_info TEXT NOT NULL;
    END IF;
    
    -- 确保status列存在且有默认值
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'resources' AND column_name = 'status') THEN
        ALTER TABLE resources ADD COLUMN status TEXT NOT NULL DEFAULT 'PENDING';
    END IF;
    
    -- 确保created_at列存在且有默认值
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'resources' AND column_name = 'created_at') THEN
        ALTER TABLE resources ADD COLUMN created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP;
    END IF;
    
    -- 确保updated_at列存在且有默认值
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'resources' AND column_name = 'updated_at') THEN
        ALTER TABLE resources ADD COLUMN updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP;
    END IF;
    
    -- 确保total_count列存在
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'resources' AND column_name = 'total_count') THEN
        ALTER TABLE resources ADD COLUMN total_count INTEGER;
    END IF;
    
    -- 确保has_pending_supplement列存在
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'resources' AND column_name = 'has_pending_supplement') THEN
        ALTER TABLE resources ADD COLUMN has_pending_supplement BOOLEAN;
    END IF;
    
    -- 确保images列存在
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'resources' AND column_name = 'images') THEN
        ALTER TABLE resources ADD COLUMN images JSONB;
    END IF;
    
    -- 确保videos列存在
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'resources' AND column_name = 'videos') THEN
        ALTER TABLE resources ADD COLUMN videos JSONB;
    END IF;
    
    -- 确保tags列存在
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'resources' AND column_name = 'tags') THEN
        ALTER TABLE resources ADD COLUMN tags TEXT[];
    END IF;
    
    -- 确保poster_image列存在
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'resources' AND column_name = 'poster_image') THEN
        ALTER TABLE resources ADD COLUMN poster_image TEXT;
    END IF;
    
    -- 确保author列存在
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'resources' AND column_name = 'author') THEN
        ALTER TABLE resources ADD COLUMN author TEXT;
    END IF;
    
    -- 确保source列存在
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'resources' AND column_name = 'source') THEN
        ALTER TABLE resources ADD COLUMN source TEXT;
    END IF;
    
    -- 确保必要的索引存在
    IF NOT EXISTS (SELECT 1 FROM pg_indexes WHERE tablename = 'resources' AND indexname = 'idx_resources_status') THEN
        CREATE INDEX idx_resources_status ON resources(status);
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM pg_indexes WHERE tablename = 'resources' AND indexname = 'idx_resources_created_at') THEN
        CREATE INDEX idx_resources_created_at ON resources(created_at DESC);
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM pg_indexes WHERE tablename = 'resources' AND indexname = 'idx_resources_media_type') THEN
        CREATE INDEX idx_resources_media_type ON resources(media_type);
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM pg_indexes WHERE tablename = 'resources' AND indexname = 'idx_resources_is_local') THEN
        CREATE INDEX idx_resources_is_local ON resources(is_local);
    END IF;
END
$$;

-- 确保encryption_keys表的列和约束正确
DO $$
BEGIN
    -- 确保resource_id列存在且有外键约束
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'encryption_keys' AND column_name = 'resource_id') THEN
        ALTER TABLE encryption_keys ADD COLUMN resource_id INTEGER NOT NULL REFERENCES resources(id) ON DELETE CASCADE;
    END IF;
    
    -- 确保key_hash列存在
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'encryption_keys' AND column_name = 'key_hash') THEN
        ALTER TABLE encryption_keys ADD COLUMN key_hash TEXT NOT NULL;
    END IF;
    
    -- 确保ukey_info列存在
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'encryption_keys' AND column_name = 'ukey_info') THEN
        ALTER TABLE encryption_keys ADD COLUMN ukey_info TEXT NOT NULL;
    END IF;
    
    -- 确保created_at列存在且有默认值
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'encryption_keys' AND column_name = 'created_at') THEN
        ALTER TABLE encryption_keys ADD COLUMN created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP;
    END IF;
    
    -- 确保必要的索引存在
    IF NOT EXISTS (SELECT 1 FROM pg_indexes WHERE tablename = 'encryption_keys' AND indexname = 'idx_encryption_keys_resource_id') THEN
        CREATE INDEX idx_encryption_keys_resource_id ON encryption_keys(resource_id);
    END IF;
END
$$;

-- 确保其他必要的表存在（如果应用需要的话）
-- 这里可以添加其他表的创建或检查逻辑

-- 确保admin用户存在
DO $$
BEGIN
    IF NOT EXISTS (SELECT FROM users WHERE username = 'admin') THEN
        INSERT INTO users (username, hashed_password, is_admin) VALUES 
        ('admin', '$2a$10$N9qo8uLOickgx2ZMRZoMyeIjZAgcfl7p92ldGxad68LJZdL17lhWy', TRUE);
    END IF;
END
$$;

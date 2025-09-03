-- Создание расширения для UUID если не существует
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Создание таблицы пользователей
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(255) NOT NULL UNIQUE, -- Уникальное имя пользователя
    password_hash VARCHAR(255) NOT NULL,   -- Хеш пароля  
    first_name VARCHAR(255) NULL,
    last_name VARCHAR(255) NULL,
    email VARCHAR(320) NULL UNIQUE, -- RFC 5321 максимальная длина email
    user_type_id INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE NULL,
    
    -- Внешний ключ на таблицу типов пользователей
    CONSTRAINT fk_user_type FOREIGN KEY (user_type_id) REFERENCES user_types(id)
);

-- Создание индексов для улучшения производительности
CREATE UNIQUE INDEX IF NOT EXISTS idx_users_username_unique ON users(username) WHERE deleted_at IS NULL;
CREATE UNIQUE INDEX IF NOT EXISTS idx_users_email_unique ON users(email) WHERE email IS NOT NULL AND deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_users_user_type ON users(user_type_id);
CREATE INDEX IF NOT EXISTS idx_users_deleted_at ON users(deleted_at);

-- Создание функции для автоматического обновления updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Создание триггера для автоматического обновления updated_at
CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

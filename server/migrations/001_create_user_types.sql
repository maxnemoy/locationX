-- Создание таблицы типов пользователей
CREATE TABLE IF NOT EXISTS user_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE
);

-- Добавление базовых типов пользователей
INSERT INTO user_types (name) VALUES 
    ('default'), 
    ('moderator'), 
    ('admin'),
    ('owner'),
    ('god');

-- Создание первого пользователя
-- Пароль 'qwerty' захеширован с помощью bcrypt (cost=12)
-- Хеш: $2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewKyAh1pTRQF.lne

INSERT INTO users (username, password_hash, first_name, last_name, email, user_type_id, created_at, updated_at)
VALUES (
    'maxnemoy',
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewKyAh1pTRQF.lne', -- bcrypt хеш для 'qwerty'
    'Max',
    'Nemoy', 
    'maxnemoy@gmail.com',
    (SELECT id FROM user_types WHERE name = 'god'), -- Получаем ID типа 'god'
    NOW(),
    NOW()
);

-- Проверка что пользователь создан
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM users WHERE username = 'maxnemoy') THEN
        RAISE NOTICE 'Пользователь maxnemoy успешно создан с типом: %', 
            (SELECT ut.name FROM users u 
             JOIN user_types ut ON u.user_type_id = ut.id 
             WHERE u.username = 'maxnemoy');
    ELSE
        RAISE EXCEPTION 'Ошибка создания пользователя maxnemoy';
    END IF;
END $$;

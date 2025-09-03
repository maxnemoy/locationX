# 🗄️ Настройка миграций базы данных

## Структура миграций

### 📄 Файлы миграций:

1. **001_create_user_types.sql** - Создает таблицу типов пользователей:
   - default (id: 1)
   - moderator (id: 2) 
   - admin (id: 3)
   - owner (id: 4)
   - **god (id: 5)** ⭐

2. **002_create_users.sql** - Создает основную таблицу пользователей:
   - `id` - UUID (автогенерация)
   - `username` - VARCHAR(255), уникальное, обязательное
   - `password_hash` - VARCHAR(255), обязательное
   - `first_name`, `last_name` - опциональные поля
   - `email` - VARCHAR(320), уникальное, опциональное
   - `user_type_id` - связь с user_types (default = 1)
   - `created_at`, `updated_at` - автоматические timestamps
   - `deleted_at` - для soft delete

3. **003_create_first_user.sql** - Создает первого пользователя:
   - **Username:** `maxnemoy`
   - **Password:** `qwerty` (захеширован bcrypt)
   - **Email:** `maxnemoy@gmail.com`
   - **Тип:** `god` (id: 5)

## 🚀 Запуск миграций

### Вариант 1: Через SQLx CLI

1. **Установите SQLx CLI:**
```bash
cargo install sqlx-cli --no-default-features --features rustls,postgres
```

2. **Настройте .env:**
```env
DATABASE_URL=postgresql://postgres:password@localhost:5432/locationx_db
```

3. **Запустите миграции:**
```bash
sqlx migrate run
```

### Вариант 2: Через код (в main.rs)

```rust
use server::infrastructure::migrations::{run_migrations, ensure_database_exists};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL должна быть установлена");
    
    // Создать БД если не существует
    ensure_database_exists(&database_url).await?;
    
    // Подключиться к БД  
    let pool = sqlx::PgPool::connect(&database_url).await?;
    
    // Запустить миграции
    run_migrations(&pool).await?;
    
    // Запустить сервер...
    
    Ok(())
}
```

## 🔐 Первый пользователь

После запуска миграций создается пользователь:

| Поле | Значение |
|------|----------|
| **Username** | `maxnemoy` |
| **Password** | `qwerty` |
| **Email** | `maxnemoy@gmail.com` |
| **Тип** | `god` (максимальные права) |

### Тестирование авторизации:

```bash
curl -X POST http://localhost:8080/v1/login \
  -H "Content-Type: application/json" \
  -d '{"username": "maxnemoy", "password": "qwerty"}'
```

**Ожидаемый ответ:**
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "...",
    "username": "maxnemoy",
    "first_name": "Max",
    "last_name": "Nemoy",
    "email": "maxnemoy@gmail.com",
    "user_type_id": 5
  }
}
```

## 🔧 Индексы и ограничения

### Уникальные индексы:
- `idx_users_username_unique` - username (где deleted_at IS NULL)
- `idx_users_email_unique` - email (где email IS NOT NULL AND deleted_at IS NULL)

### Внешние ключи:
- `fk_user_type` - users.user_type_id → user_types.id

### Триггеры:
- `update_users_updated_at` - автообновление updated_at при изменении записи

## 📊 Структура связей

```
user_types (id, name)
    ↓ (user_type_id)
users (id, username, password_hash, email, user_type_id, ...)
```

### JOIN пример:
```sql
SELECT u.username, u.email, ut.name as user_type
FROM users u
JOIN user_types ut ON u.user_type_id = ut.id
WHERE u.username = 'maxnemoy';
```

## ⚠️ Безопасность

- Пароли хешируются bcrypt с cost=12
- Username и email уникальные (с учетом soft delete)
- Поле password_hash никогда не возвращается в API ответах
- Soft delete через deleted_at поле

## 🔄 Следующие шаги

1. **Подключите реальную БД:** Замените InMemoryRepository на PostgreSQL
2. **Добавьте middleware:** Активируйте аутентификацию для защищенных endpoints  
3. **Реализуйте CRUD:** Полные операции с пользователями через БД
4. **Тесты:** Напишите тесты для миграций и API

База данных готова к использованию! 🎉

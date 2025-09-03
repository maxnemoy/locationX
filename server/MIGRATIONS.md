# Инструкции по работе с миграциями

## Установка SQLx CLI

Для работы с миграциями установите SQLx CLI:

```bash
cargo install sqlx-cli --no-default-features --features rustls,postgres
```

## Создание новой миграции

```bash
sqlx migrate add create_new_table
```

## Запуск миграций

### Через CLI (требует DATABASE_URL в .env):
```bash
sqlx migrate run
```

### Через код приложения:

Добавьте в ваш main.rs или другой entry point:

```rust
use server::infrastructure::migrations::{run_migrations, ensure_database_exists};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Загрузка переменных окружения
    dotenvy::dotenv().ok();
    
    // Создание URL базы данных
    let host = std::env::var("PGSQL_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = std::env::var("PGSQL_PORT").unwrap_or_else(|_| "5432".to_string());
    let user = std::env::var("PGSQL_USER").unwrap_or_else(|_| "postgres".to_string());
    let password = std::env::var("PGSQL_PASSWD").unwrap_or_else(|_| "password".to_string());
    let database = std::env::var("PGSQL_DB").unwrap_or_else(|_| "test_db".to_string());
    
    let database_url = format!(
        "postgresql://{}:{}@{}:{}/{}",
        user, password, host, port, database
    );
    
    // Убедитесь что БД существует
    ensure_database_exists(&database_url).await?;
    
    // Подключитесь к БД
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    
    // Запустите миграции
    run_migrations(&pool).await?;
    
    // Ваш код приложения...
    
    Ok(())
}
```

## Откат миграций

```bash
sqlx migrate revert
```

## Проверка статуса миграций

```bash
sqlx migrate info
```

## Структура созданных таблиц

### Таблица `user_types`
- `id` - автоинкремент PRIMARY KEY
- `name` - уникальное имя типа пользователя

Предустановленные типы:
1. обычный (по умолчанию)
2. администратор  
3. модератор

### Таблица `users`
- `id` - UUID v4 (автогенерируемый)
- `first_name` - имя (опциональное)  
- `last_name` - фамилия (опциональное)
- `email` - email (опциональное, уникальное)
- `user_type_id` - ссылка на user_types (по умолчанию 1)
- `created_at` - дата создания (автоматически)
- `updated_at` - дата обновления (автоматически через триггер)
- `deleted_at` - дата удаления (по умолчанию NULL - soft delete)

## Настройка переменных окружения

Создайте файл `.env` на основе `example.env`:

```bash
PGSQL_HOST=localhost
PGSQL_PORT=5432
PGSQL_USER=postgres
PGSQL_PASSWD=password
PGSQL_DB=locationx_db
```

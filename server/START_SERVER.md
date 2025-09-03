# 🚀 Запуск сервера

## Подготовка к запуску

1. **Настройте файл окружения**
   Создайте файл `.env` в корне проекта:
   ```env
   PORT=8080
   HOST=127.0.0.1
   PGSQL_HOST=localhost
   PGSQL_PORT=5432
   PGSQL_USER=postgres
   PGSQL_PASSWD=password
   PGSQL_DB=locationx_db
   JWT_SECRET=your_super_secret_jwt_key_change_this_in_production
   DATABASE_URL=postgresql://postgres:password@localhost:5432/locationx_db
   ```

2. **Установите зависимости**
   ```bash
   cargo build
   ```

3. **Запустите сервер**
   ```bash
   cargo run
   ```

   Сервер будет доступен по адресу: http://127.0.0.1:8080

## Проверка работы

### Проверьте статус сервера:
```bash
curl http://localhost:8080/v1/status/server
```

### Проверьте гостевой endpoint:
```bash
curl http://localhost:8080/v1/guest
```

### Тест аутентификации:
```bash
curl -X POST http://localhost:8080/v1/login \
  -H "Content-Type: application/json" \
  -d '{"email": "test@example.com", "password": "password123"}'
```

## Структура API

### Новые эндпоинты:
- `PUT /v1/register` - регистрация пользователя
- `POST /v1/login` - авторизация (тестовые данные: test@example.com / password123)
- `POST /v1/token` - обновление access токена
- `GET /v1/guest` - открытый endpoint
- `GET /v1/user` - информация о пользователе
- `PATCH /v1/user` - обновление пользователя

### Старые эндпоинты (совместимость):
- `GET /v1/status/server` - статус сервера
- `GET /v1/status/db` - статус базы данных
- `POST /v1/users` - создать пользователя (старый формат)
- `GET /v1/users` - список пользователей
- `GET /v1/users/{id}` - получить пользователя по ID
- `DELETE /v1/users/{id}` - удалить пользователя

## База данных

### Миграции (если нужны):
```bash
# Установите SQLx CLI
cargo install sqlx-cli --no-default-features --features rustls,postgres

# Запустите миграции
sqlx migrate run
```

### Структура таблиц:
- `user_types` - типы пользователей (default, admin, moderator)  
- `users` - основная таблица пользователей с UUID, паролем и метаданными

## Примечания

- 🔧 Сервер работает в режиме разработки с mock-данными
- 🔐 JWT токены: access (15 мин), refresh (30 дней)  
- 🔑 Тестовые учетные данные: test@example.com / password123
- 📄 Подробную документацию API см. в `API_ENDPOINTS.md`

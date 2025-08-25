# LocationX Server

Простое веб-приложение на Actix-web с поддержкой PostgreSQL, построенное по принципам чистой архитектуры.

## Архитектура

Проект следует принципам Clean Architecture с четким разделением слоев:

```
src/
├── main.rs              # Точка входа приложения
├── lib.rs               # Модули библиотеки
├── domain/              # ДОМЕННЫЙ СЛОЙ
│   ├── mod.rs
│   ├── entities.rs      # Сущности (DbStatus, PingResponse)
│   └── traits.rs        # Интерфейсы (HealthService, DatabaseHealthChecker)
├── application/         # СЛОЙ ПРИЛОЖЕНИЯ  
│   ├── mod.rs
│   └── services.rs      # Сервисы бизнес-логики (HealthServiceImpl)
├── infrastructure/      # ИНФРАСТРУКТУРНЫЙ СЛОЙ
│   ├── mod.rs
│   └── database.rs      # Подключение к PostgreSQL (PostgresHealthChecker)
└── presentation/        # СЛОЙ ПРЕДСТАВЛЕНИЯ
    ├── mod.rs
    ├── handlers.rs      # HTTP обработчики
    └── routes.rs        # Маршрутизация
```

### Слои архитектуры

#### 🟦 Domain (Доменный слой)
- **Entities**: Бизнес-сущности (`DbStatus`, `PingResponse`)
- **Traits**: Интерфейсы (`HealthService`, `DatabaseHealthChecker`)
- Не зависит от других слоев
- Содержит чистую бизнес-логику

#### 🟩 Application (Слой приложения)
- **Services**: Реализация бизнес-логики (`HealthServiceImpl`)
- Оркестрирует взаимодействие между слоями
- Зависит только от доменного слоя

#### 🟨 Infrastructure (Инфраструктурный слой)  
- **Database**: Подключение к PostgreSQL (`PostgresHealthChecker`)
- Реализует интерфейсы доменного слоя
- Содержит внешние зависимости

#### 🟪 Presentation (Слой представления)
- **Handlers**: HTTP обработчики запросов
- **Routes**: Конфигурация маршрутов
- Точка входа для внешних запросов

## Endpoints

### Health Status (v1/status)
- `GET ` - проверка работоспособности сервера
- `GET /v1/status/db` - проверка статуса подключения к PostgreSQL

### User Management (v1/user)
- `POST /v1/user` - создать нового пользователя
- `GET /v1/user` - получить список всех пользователей  
- `GET /v1/user/{id}` - получить пользователя по ID
- `DELETE /v1/user/{id}` - удалить пользователя по ID

### Примеры запросов

**Создание пользователя:**
```bash
curl -X POST http://localhost:5000/v1/user \
  -H "Content-Type: application/json" \
  -d '{"username": "john_doe", "email": "john@example.com"}'
```

**Получение всех пользователей:**
```bash
curl http://localhost:5000/v1/user
```

**Получение конкретного пользователя:**
```bash  
curl http://localhost:5000/v1/user/1
```

## Переменные окружения (.env)

```env
# Сервер
PORT=
HOST=

# PostgreSQL
PGSQL_HOST=
PGSQL_PORT=
PGSQL_USER=
PGSQL_PASSWD=
PGSQL_DB=
```

## Запуск

```bash
cargo run
```

## Зависимости

- `actix-web` - веб-фреймворк
- `sqlx` - асинхронный PostgreSQL драйвер  
- `serde` - сериализация
- `dotenvy` - загрузка .env файлов
- `async-trait` - async трейты

## Преимущества архитектуры

✅ **Разделение ответственности** - каждый слой имеет четкую роль  
✅ **Инверсия зависимостей** - высокоуровневые слои не зависят от низкоуровневых  
✅ **Тестируемость** - легко мокать зависимости через трейты  
✅ **Масштабируемость** - простое добавление новых фич  
✅ **Независимость от фреймворка** - бизнес-логика не привязана к Actix-web

# API Endpoints документация

## 🚀 Настройка

Добавьте в ваш `.env` файл:

```env
JWT_SECRET=your_super_secret_key_here
```

## 📋 Список эндпоинтов

### 🔓 Публичные эндпоинты (не требуют авторизации)

#### PUT /v1/register - Регистрация пользователя
Создает нового пользователя в системе.

**Запрос:**
```json
{
  "username": "ivan_petrov",
  "first_name": "Иван",
  "last_name": "Петров", 
  "email": "ivan@example.com",
  "password": "password123"
}
```

**Ответ (201):**
```json
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "email": "ivan@example.com",
  "message": "Пользователь успешно зарегистрирован"
}
```

#### POST /v1/login - Авторизация  
Авторизует пользователя по username и паролю и возвращает JWT токены.

**Запрос:**
```json
{
  "username": "maxnemoy",
  "password": "qwerty"
}
```

**Ответ (200):**
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "username": "maxnemoy",
    "first_name": "Max",
    "last_name": "Nemoy", 
    "email": "maxnemoy@gmail.com",
    "user_type_id": 5
  }
}
```

#### POST /v1/token - Обновление токена
Обновляет access токен используя refresh токен.

**Запрос:**
```json
{
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

**Ответ (200):**
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

#### GET /v1/guest - Гостевой доступ
Открытый эндпоинт для неавторизованных пользователей.

**Ответ (200):**
```json
{
  "message": "Добро пожаловать! Это открытый endpoint",
  "timestamp": "2024-01-01T12:00:00Z"
}
```

### 🔒 Защищенные эндпоинты (требуют авторизации)

**Заголовок авторизации:**
```
Authorization: Bearer <access_token>
```

#### GET /v1/user - Информация о пользователе
Возвращает информацию о текущем авторизованном пользователе **из базы данных** на основе JWT токена.

**Заголовки:**
```
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

**Ответ (200):**
```json
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "username": "maxnemoy",
  "first_name": "Max",
  "last_name": "Nemoy", 
  "email": "maxnemoy@gmail.com",
  "user_type_id": 5
}
```

**Ошибки:**
- `401 Unauthorized` - отсутствует или недействительный токен
- `404 Not Found` - пользователь не найден в БД

#### PATCH /v1/user - Обновление пользователя
Обновляет поля текущего пользователя **в базе данных** на основе JWT токена.

**Заголовки:**
```
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

**Запрос:**
```json
{
  "first_name": "Новое Имя",
  "last_name": "Новая Фамилия",
  "email": "newemail@example.com"
}
```

**Ответ (200):**
```json
{
  "user": {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "username": "maxnemoy",
    "first_name": "Новое Имя",
    "last_name": "Новая Фамилия", 
    "email": "newemail@example.com",
    "user_type_id": 5
  },
  "message": "Пользователь успешно обновлен"
}
```

**Ошибки:**
- `401 Unauthorized` - отсутствует или недействительный токен
- `404 Not Found` - пользователь не найден в БД

### 🩺 Служебные эндпоинты

#### GET /v1/status/server - Статус сервера
```json
{
  "status": "ok",
  "message": "pong"
}
```

#### GET /v1/status/db - Статус базы данных
```json
{
  "status": "ok", 
  "message": "База данных доступна",
  "connected": true
}
```

## 🔐 JWT Токены

- **Access Token:** Действует 15 минут
- **Refresh Token:** Действует 30 дней

## ❌ Ошибки

### 400 Bad Request
```json
{
  "error": "Email обязателен"
}
```

### 401 Unauthorized
```json
{
  "error": "Требуется авторизация"
}
```

### 500 Internal Server Error
```json
{
  "error": "Ошибка хеширования пароля"
}
```

## 🧪 Тестирование API

### Пример тестирования с curl:

1. **Авторизация:**
```bash
curl -X POST http://localhost:8080/v1/login \
  -H "Content-Type: application/json" \
  -d '{"username": "maxnemoy", "password": "qwerty"}'
```

2. **Получение информации о пользователе:**
```bash
# Сначала получите токен через /v1/login, затем используйте его:
curl -X GET http://localhost:8080/v1/user \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
```

3. **Гостевой доступ:**
```bash
curl -X GET http://localhost:8080/v1/guest
```

## 📝 Примечания

- Все эндпоинты возвращают JSON
- Пароли хешируются с использованием bcrypt
- UUID v4 используется как идентификатор пользователей
- Система поддерживает типы пользователей (default, admin, moderator)
- Реализована система soft delete (поле deleted_at)

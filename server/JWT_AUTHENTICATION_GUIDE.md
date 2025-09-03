# 🔐 JWT Аутентификация - Руководство

## 🎯 **Как это теперь работает:**

### **ДО (случайные данные):**
```rust
// Возвращал случайный UUID
let user_info = UserInfo {
    id: Uuid::new_v4(), // ❌ Случайный ID каждый раз
    username: "demo_user".to_string(), // ❌ Фиксированные данные
};
```

### **ПОСЛЕ (реальная аутентификация):**
```rust
// 1. Извлекаем user_id из JWT токена
let user_id = extract_user_id_from_request(&req, &jwt_service).await?;

// 2. Ищем пользователя в БД по этому ID
let user = user_auth_repository.find_by_id(user_id).await?;

// 3. Возвращаем РЕАЛЬНЫЕ данные из БД
let user_info = UserInfo {
    id: user.id,           // ✅ Реальный ID из БД
    username: user.username, // ✅ Реальные данные пользователя
    first_name: user.first_name,
    // ... другие поля из БД
};
```

## 🔄 **Полный флоу аутентификации:**

### 1. **Авторизация**
```bash
curl -X POST http://localhost:8080/v1/login \
  -H "Content-Type: application/json" \
  -d '{"username": "maxnemoy", "password": "qwerty"}'
```

**Ответ:**
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": { ... }
}
```

### 2. **Получение информации о пользователе**
```bash
curl -X GET http://localhost:8080/v1/user \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

**Что происходит внутри:**
1. Извлекается токен из заголовка `Authorization: Bearer <token>`
2. JWT токен декодируется → извлекается `user_id`
3. По `user_id` выполняется SQL запрос к БД
4. Возвращаются **реальные данные пользователя** из PostgreSQL

### 3. **Обновление пользователя**
```bash
curl -X PATCH http://localhost:8080/v1/user \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..." \
  -H "Content-Type: application/json" \
  -d '{"first_name": "Новое имя", "last_name": "Новая фамилия"}'
```

**Что происходит внутри:**
1. Извлекается `user_id` из JWT токена
2. Выполняется SQL UPDATE для этого пользователя
3. Возвращаются **обновленные данные** из БД

## 🔒 **Система безопасности:**

### **JWT Токен содержит:**
```json
{
  "sub": "123e4567-e89b-12d3-a456-426614174000", // user_id
  "email": "maxnemoy@gmail.com",
  "exp": 1703123456, // срок действия
  "iat": 1703037056, // время создания  
  "token_type": "access" // тип токена
}
```

### **Проверка токена:**
```rust
// 1. Валидация JWT подписи
let token_data = jwt_service.verify_token(token)?;

// 2. Извлечение user_id
let user_id = Uuid::parse_str(&token_data.claims.sub)?;

// 3. Поиск пользователя в БД
let user = user_auth_repository.find_by_id(user_id).await?;
```

### **Ошибки аутентификации:**

**401 Unauthorized:**
- Отсутствует заголовок `Authorization`
- Неверный формат токена (не `Bearer ...`)
- Истекший или недействительный JWT токен

**404 Not Found:**
- Пользователь с ID из токена не найден в БД (был удален)

**400 Bad Request:**
- Неверный формат заголовка Authorization

## 📊 **Преимущества новой системы:**

### ✅ **Безопасность:**
- Каждый запрос привязан к конкретному пользователю
- Невозможно получить чужие данные
- Токены имеют срок действия

### ✅ **Актуальность данных:**
- Данные всегда извлекаются из БД
- Отражают текущее состояние пользователя
- Изменения видны сразу

### ✅ **Целостность:**
- Если пользователь удален из БД → 404 ошибка
- Если токен подделан → 401 ошибка  
- Если токен истек → 401 ошибка

## 🧪 **Тестирование:**

### 1. **Получите токен:**
```bash
TOKEN=$(curl -s -X POST http://localhost:8080/v1/login \
  -H "Content-Type: application/json" \
  -d '{"username": "maxnemoy", "password": "qwerty"}' | \
  jq -r '.access_token')
```

### 2. **Используйте токен:**
```bash
curl -X GET http://localhost:8080/v1/user \
  -H "Authorization: Bearer $TOKEN"
```

### 3. **Обновите данные:**
```bash
curl -X PATCH http://localhost:8080/v1/user \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"first_name": "Обновленное имя"}'
```

## 🎉 **Результат:**

**Теперь GET /v1/user и PATCH /v1/user возвращают данные РЕАЛЬНОГО пользователя из PostgreSQL БД на основе JWT токена, а не случайные значения!**

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Serialize, Debug, Clone)]
pub struct DbStatus {
    pub status: String,
    pub message: String,
    pub connected: bool,
}

impl DbStatus {
    pub fn connected(message: String) -> Self {
        Self {
            status: "ok".to_string(),
            message,
            connected: true,
        }
    }

    pub fn disconnected(message: String) -> Self {
        Self {
            status: "error".to_string(),
            message,
            connected: false,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct PingResponse {
    pub status: String,
    pub message: String,
}

impl Default for PingResponse {
    fn default() -> Self {
        Self {
            status: "ok".to_string(),
            message: "pong".to_string(),
        }
    }
}

// Обновленная структура пользователя с UUID и новыми полями
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>, 
    pub email: Option<String>,
    pub user_type_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    // password_hash не включаем в сериализацию из соображений безопасности
}

// Структуры для регистрации пользователя
#[derive(Deserialize, Debug)]
pub struct RegisterUserRequest {
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct RegisterUserResponse {
    pub id: Uuid,
    pub email: String,
    pub message: String,
}

// Структуры для авторизации
#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserInfo,
}

#[derive(Serialize, Debug, Clone)]
pub struct UserInfo {
    pub id: Uuid,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub user_type_id: i32,
}

// Структуры для обновления токена
#[derive(Deserialize, Debug)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Serialize, Debug)]
pub struct RefreshTokenResponse {
    pub access_token: String,
}

// Структуры для обновления пользователя
#[derive(Deserialize, Debug)]
pub struct UpdateUserRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct UpdateUserResponse {
    pub user: UserInfo,
    pub message: String,
}

// Структуры для гостевого доступа
#[derive(Serialize, Debug)]
pub struct GuestResponse {
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

// Структуры для JWT токенов
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String, // user_id
    pub email: String,
    pub exp: usize,
    pub iat: usize,
    pub token_type: String, // "access" или "refresh"
}


#[derive(Deserialize, Debug)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
}

#[derive(Serialize, Debug)]
pub struct CreateUserResponse {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct UsersListResponse {
    pub users: Vec<User>,
    pub total: usize,
}

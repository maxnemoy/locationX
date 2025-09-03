use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::{domain::{entities::{LoginRequest, LoginResponse, UserInfo}, traits::UserAuthRepository}, infrastructure::jwt::jwt_service::JwtService};

pub async fn handler(
    jwt_service: web::Data<JwtService>,
    user_auth_repository: web::Data<Arc<dyn UserAuthRepository + Send + Sync>>,
    request_data: web::Json<LoginRequest>,
) -> impl Responder {
    let req = request_data.into_inner();

    // Ищем пользователя в БД
    let user = match user_auth_repository.find_by_username(&req.username).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Неверный username или пароль"
            }));
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Ошибка поиска пользователя: {}", e)
            }));
        }
    };

    // Получаем хеш пароля
    let password_hash = match user_auth_repository.get_password_hash(&req.username).await {
        Ok(Some(hash)) => hash,
        Ok(None) => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Неверный username или пароль"
            }));
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Ошибка получения пароля: {}", e)
            }));
        }
    };

    // Проверяем пароль
    let password_valid = match bcrypt::verify(req.password.as_bytes(), &password_hash) {
        Ok(valid) => valid,
        Err(_) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Ошибка проверки пароля"
            }));
        }
    };

    if !password_valid {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Неверный username или пароль"
        }));
    }

    // Генерация токенов
    let access_token =
        match jwt_service.generate_access_token(user.id, user.email.as_deref().unwrap_or("")) {
            Ok(token) => token,
            Err(_) => {
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Ошибка создания access токена"
                }));
            }
        };

    let refresh_token =
        match jwt_service.generate_refresh_token(user.id, user.email.as_deref().unwrap_or("")) {
            Ok(token) => token,
            Err(_) => {
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Ошибка создания refresh токена"
                }));
            }
        };

    let response = LoginResponse {
        access_token,
        refresh_token,
        user: UserInfo {
            id: user.id,
            username: user.username,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            user_type_id: user.user_type_id,
        },
    };

    HttpResponse::Ok().json(response)
}
use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::domain::{entities::{RegisterUserRequest, RegisterUserResponse}, traits::UserAuthRepository};

pub async fn handler(
    user_auth_repository: web::Data<Arc<dyn UserAuthRepository + Send + Sync>>,
    request_data: web::Json<RegisterUserRequest>,
) -> impl Responder {
    let req = request_data.into_inner();

    // Валидация username
    if req.username.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Username обязателен"
        }));
    }

    // Валидация email
    if req.email.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Email обязателен"
        }));
    }

    // Валидация пароля
    if req.password.len() < 6 {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Пароль должен быть не менее 6 символов"
        }));
    }

    // Проверяем что пользователь с таким username не существует
    match user_auth_repository.find_by_username(&req.username).await {
        Ok(Some(_)) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Пользователь с таким username уже существует"
            }));
        }
        Ok(None) => {} // Все хорошо, можно создавать
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Ошибка проверки пользователя: {}", e)
            }));
        }
    }

    // Создаем пользователя в БД
    match user_auth_repository
        .create_user_with_password(
            &req.username,
            &req.password,
            req.first_name.as_deref(),
            req.last_name.as_deref(),
            Some(&req.email),
            None, // используем значение по умолчанию
        )
        .await
    {
        Ok(user) => HttpResponse::Created().json(RegisterUserResponse {
            id: user.id,
            email: req.email,
            message: "Пользователь успешно зарегистрирован".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Ошибка создания пользователя: {}", e)
        })),
    }
}
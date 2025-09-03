use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, Responder, web};

use crate::{
    domain::{entities::UserInfo, traits::UserAuthRepository},
    infrastructure::jwt::{
        extract_user_uuid::from_request as extract_user_uuid, jwt_service::JwtService,
    },
};

// GET /v1/user - получить информацию о пользователе
pub async fn handler(
    req: HttpRequest,
    jwt_service: web::Data<JwtService>,
    user_auth_repository: web::Data<Arc<dyn UserAuthRepository + Send + Sync>>,
) -> impl Responder {
    // Извлекаем user_id из JWT токена
    let user_id = match extract_user_uuid(&req, &jwt_service).await {
        Ok(id) => id,
        Err(response) => return response,
    };

    // Ищем пользователя в БД по ID из токена
    match user_auth_repository.find_by_id(user_id).await {
        Ok(Some(user)) => {
            let user_info = UserInfo {
                id: user.id,
                username: user.username,
                first_name: user.first_name,
                last_name: user.last_name,
                email: user.email,
                user_type_id: user.user_type_id,
            };
            HttpResponse::Ok().json(user_info)
        }
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Пользователь не найден"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Ошибка поиска пользователя: {}", e)
        })),
    }
}

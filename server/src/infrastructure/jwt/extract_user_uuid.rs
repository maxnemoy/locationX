use actix_web::{HttpRequest, HttpResponse};
use uuid::Uuid;

use crate::infrastructure::jwt::jwt_service::JwtService;

// Вспомогательная функция для извлечения user_id из JWT токена
pub async fn from_request(
    req: &HttpRequest,
    jwt_service: &JwtService,
) -> Result<Uuid, HttpResponse> {
    // Извлекаем токен из заголовка Authorization
    let auth_header = match req.headers().get("Authorization") {
        Some(header) => header,
        None => {
            return Err(HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Требуется заголовок Authorization"
            })));
        }
    };

    let auth_str = match auth_header.to_str() {
        Ok(str) => str,
        Err(_) => {
            return Err(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Неверный формат заголовка Authorization"
            })));
        }
    };

    if !auth_str.starts_with("Bearer ") {
        return Err(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Токен должен начинаться с 'Bearer '"
        })));
    }

    let token = &auth_str[7..]; // Убираем "Bearer "

    // Декодируем JWT токен
    match jwt_service.get_user_id_from_token(token) {
        Ok(user_id) => Ok(user_id),
        Err(_) => Err(HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Недействительный или истекший токен"
        }))),
    }
}

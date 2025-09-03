use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use crate::{domain::entities::{RefreshTokenRequest, RefreshTokenResponse}, infrastructure::jwt::jwt_service::JwtService};

pub async fn handler(
    jwt_service: web::Data<JwtService>,
    request_data: web::Json<RefreshTokenRequest>,
) -> impl Responder {
    let req = request_data.into_inner();

    // Проверяем refresh токен
    match jwt_service.verify_token(&req.refresh_token) {
        Ok(token_data) => {
            // Проверяем тип токена
            if token_data.claims.token_type != "refresh" {
                return HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Неверный тип токена"
                }));
            }

            // Парсим user_id
            let user_id = match Uuid::parse_str(&token_data.claims.sub) {
                Ok(id) => id,
                Err(_) => {
                    return HttpResponse::BadRequest().json(serde_json::json!({
                        "error": "Неверный формат пользователя в токене"
                    }));
                }
            };

            // Генерируем новый access токен
            match jwt_service.generate_access_token(user_id, &token_data.claims.email) {
                Ok(access_token) => HttpResponse::Ok().json(RefreshTokenResponse { access_token }),
                Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Ошибка создания access токена"
                })),
            }
        }
        Err(_) => HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Недействительный refresh токен"
        })),
    }
}

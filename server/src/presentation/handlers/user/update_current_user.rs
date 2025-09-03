use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, Responder, web};

use crate::{
    domain::{
        entities::{UpdateUserRequest, UpdateUserResponse, UserInfo},
        traits::UserAuthRepository,
    },
    infrastructure::jwt::{
        extract_user_uuid::from_request as extract_user_uuid, jwt_service::JwtService,
    },
};

pub async fn handler(
    req: HttpRequest,
    jwt_service: web::Data<JwtService>,
    user_auth_repository: web::Data<Arc<dyn UserAuthRepository + Send + Sync>>,
    request_data: web::Json<UpdateUserRequest>,
) -> impl Responder {
    let update_req = request_data.into_inner();

    // Извлекаем user_id из JWT токена
    let user_id = match extract_user_uuid(&req, &jwt_service).await {
        Ok(id) => id,
        Err(response) => return response,
    };

    // Обновляем пользователя в БД
    match user_auth_repository
        .update_user_fields(
            user_id,
            update_req.first_name.as_deref(),
            update_req.last_name.as_deref(),
            update_req.email.as_deref(),
        )
        .await
    {
        Ok(Some(updated_user)) => {
            let user_info = UserInfo {
                id: updated_user.id,
                username: updated_user.username,
                first_name: updated_user.first_name,
                last_name: updated_user.last_name,
                email: updated_user.email,
                user_type_id: updated_user.user_type_id,
            };

            HttpResponse::Ok().json(UpdateUserResponse {
                user: user_info,
                message: "Пользователь успешно обновлен".to_string(),
            })
        }
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Пользователь не найден"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Ошибка обновления пользователя: {}", e)
        })),
    }
}

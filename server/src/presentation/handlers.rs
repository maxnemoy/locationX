use actix_web::{web, HttpResponse, Responder};
use crate::domain::traits::{HealthService, UserService};
use crate::domain::entities::CreateUserRequest;
use std::sync::Arc;

// Health handlers
pub async fn server_status_handler(
    health_service: web::Data<Arc<dyn HealthService + Send + Sync>>,
) -> impl Responder {
    let response = health_service.status().await;
    HttpResponse::Ok().json(response)
}

pub async fn db_status_handler(
    health_service: web::Data<Arc<dyn HealthService + Send + Sync>>,
) -> impl Responder {
    let db_status = health_service.check_database().await;
    
    if db_status.connected {
        HttpResponse::Ok().json(db_status)
    } else {
        HttpResponse::ServiceUnavailable().json(db_status)
    }
}

// User handlers
pub async fn create_user_handler(
    user_service: web::Data<Arc<dyn UserService + Send + Sync>>,
    user_data: web::Json<CreateUserRequest>,
) -> impl Responder {
    match user_service.create_user(user_data.into_inner()).await {
        Ok(response) => HttpResponse::Created().json(response),
        Err(err) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": err
        })),
    }
}

pub async fn get_user_handler(
    user_service: web::Data<Arc<dyn UserService + Send + Sync>>,
    path: web::Path<u32>,
) -> impl Responder {
    let user_id = path.into_inner();
    
    match user_service.get_user(user_id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::NotFound().json(serde_json::json!({
            "error": err
        })),
    }
}

pub async fn get_all_users_handler(
    user_service: web::Data<Arc<dyn UserService + Send + Sync>>,
) -> impl Responder {
    match user_service.get_all_users().await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err
        })),
    }
}

pub async fn delete_user_handler(
    user_service: web::Data<Arc<dyn UserService + Send + Sync>>,
    path: web::Path<u32>,
) -> impl Responder {
    let user_id = path.into_inner();
    
    match user_service.delete_user(user_id).await {
        Ok(message) => HttpResponse::Ok().json(serde_json::json!({
            "message": message
        })),
        Err(err) => HttpResponse::NotFound().json(serde_json::json!({
            "error": err
        })),
    }
}

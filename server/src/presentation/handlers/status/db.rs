use std::sync::Arc;

use actix_web::{HttpResponse, Responder, web};

use crate::domain::traits::HealthService;

pub async fn handler(
    health_service: web::Data<Arc<dyn HealthService + Send + Sync>>,
) -> impl Responder {
    let db_status = health_service.check_database().await;

    if db_status.connected {
        HttpResponse::Ok().json(db_status)
    } else {
        HttpResponse::ServiceUnavailable().json(db_status)
    }
}

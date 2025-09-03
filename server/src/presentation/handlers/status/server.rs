use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::domain::traits::HealthService;

pub async fn handler(
    health_service: web::Data<Arc<dyn HealthService + Send + Sync>>,
) -> impl Responder {
    let response = health_service.status().await;
    HttpResponse::Ok().json(response)
}

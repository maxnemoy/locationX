use actix_web::{HttpResponse, Responder};
use chrono::Utc;

use crate::domain::entities::GuestResponse;

pub async fn handler() -> impl Responder {
    HttpResponse::Ok().json(GuestResponse {
        message: "Добро пожаловать! Это открытый endpoint".to_string(),
        timestamp: Utc::now(),
    })
}

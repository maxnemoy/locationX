use crate::presentation::handlers::{
    guest::guest_zone,
    status::{db, server},
    token::refresh,
    user::{auth_user, get_current_user, register_user, update_current_user},
};

use actix_web::{Scope, web};

pub fn api_v1_routes() -> Scope {
    web::scope("/v1")
        .route("/token", web::post().to(refresh::handler))
        .service(status_routes())
        .service(user_routes())
        .service(guest_routes())
}

pub fn status_routes() -> Scope {
    web::scope("/status")
        .route("/server", web::get().to(server::handler))
        .route("/db", web::get().to(db::handler))
}

// Новые API эндпоинты
pub fn user_routes() -> Scope {
    web::scope("user")
        .route("/", web::get().to(get_current_user::handler))
        .route("/", web::patch().to(update_current_user::handler))
        .route("/", web::put().to(register_user::handler))
        .route("/", web::post().to(auth_user::handler))
}

pub fn guest_routes() -> Scope {
    web::scope("guest").route("/", web::get().to(guest_zone::handler))
}

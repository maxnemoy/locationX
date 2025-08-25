use actix_web::{web, Scope};
use crate::presentation::handlers::{
    server_status_handler, 
    db_status_handler,
    create_user_handler,
    get_user_handler,
    get_all_users_handler,
    delete_user_handler,
};

pub fn status_routes() -> Scope {
    web::scope("/status")
        .route("/server", web::get().to(server_status_handler))
        .route("/db", web::get().to(db_status_handler))
}

pub fn user_routes() -> Scope {
    web::scope("/user")
        .route("", web::post().to(create_user_handler))
        .route("", web::get().to(get_all_users_handler))
        .route("/{id}", web::get().to(get_user_handler))
        .route("/{id}", web::delete().to(delete_user_handler))
}

pub fn api_v1_routes() -> Scope {
    web::scope("/v1")
        .service(status_routes())
        .service(user_routes())
}

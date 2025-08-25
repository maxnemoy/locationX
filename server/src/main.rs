use actix_web::{App, HttpServer, web};
use std::env;
use std::sync::Arc;

use server::application::services::{HealthServiceImpl, UserServiceImpl};
use server::infrastructure::{
    database::PostgresHealthChecker, user_repository::InMemoryUserRepository,
};
use server::presentation::routes::api_v1_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    // Инициализируем инфраструктурные сервисы
    let db_checker = Arc::new(PostgresHealthChecker::new().await);
    let health_service: Arc<dyn server::domain::traits::HealthService + Send + Sync> =
        Arc::new(HealthServiceImpl::new(db_checker));

    let user_repository = Arc::new(InMemoryUserRepository::new());
    let user_service: Arc<dyn server::domain::traits::UserService + Send + Sync> =
        Arc::new(UserServiceImpl::new(user_repository));

    // Получаем настройки сервера из переменных окружения
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT должен быть валидным числом");

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

    let bind_address = (host, port);
    println!(
        "Запуск сервера на http://{}:{}",
        bind_address.0, bind_address.1
    );

    // Запускаем HTTP сервер
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(health_service.clone()))
            .app_data(web::Data::new(user_service.clone()))
            .service(api_v1_routes())
    })
    .bind(bind_address)?
    .run()
    .await
}

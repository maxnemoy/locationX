use actix_web::{App, HttpServer, web};
use std::env;
use std::sync::Arc;

use server::application::services::{HealthServiceImpl, UserServiceImpl};
use server::infrastructure::{
    database::PostgresHealthChecker, 
    postgres_user_repository::PostgreSQLUserRepository,
    jwt::jwt_service::JwtService,
    migrations::{run_migrations, ensure_database_exists},
};
use server::domain::traits::UserAuthRepository;
use server::presentation::routes::api_v1_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    // Настройка базы данных
    let host = env::var("PGSQL_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("PGSQL_PORT").unwrap_or_else(|_| "5432".to_string());
    let user = env::var("PGSQL_USER").unwrap_or_else(|_| "postgres".to_string());
    let password = env::var("PGSQL_PASSWD").unwrap_or_else(|_| "password".to_string());
    let database = env::var("PGSQL_DB").unwrap_or_else(|_| "locationx_db".to_string());
    
    let database_url = format!(
        "postgresql://{}:{}@{}:{}/{}",
        user, password, host, port, database
    );

    // Убедиться что БД существует
    if let Err(e) = ensure_database_exists(&database_url).await {
        eprintln!("Ошибка создания базы данных: {}", e);
    }

    // Создать пул соединений к БД
    let db_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Не удалось подключиться к базе данных");

    // Запустить миграции
    if let Err(e) = run_migrations(&db_pool).await {
        eprintln!("Ошибка запуска миграций: {}", e);
    }

    // Инициализируем инфраструктурные сервисы
    let db_checker = Arc::new(PostgresHealthChecker::new().await);
    let health_service: Arc<dyn server::domain::traits::HealthService + Send + Sync> =
        Arc::new(HealthServiceImpl::new(db_checker));

    // Используем PostgreSQL repository вместо InMemory
    let postgres_repository = Arc::new(PostgreSQLUserRepository::new(db_pool.clone()));
    let user_repository = postgres_repository.clone();
    let user_service: Arc<dyn server::domain::traits::UserService + Send + Sync> =
        Arc::new(UserServiceImpl::new(user_repository));

    // Auth repository (тот же PostgreSQL repository, но через другой trait)
    let user_auth_repository: Arc<dyn UserAuthRepository + Send + Sync> = postgres_repository;

    // Создаем JWT сервис
    let jwt_service = JwtService::new();

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
            .app_data(web::Data::new(user_auth_repository.clone()))
            .app_data(web::Data::new(jwt_service.clone()))
            .service(api_v1_routes())
    })
    .bind(bind_address)?
    .run()
    .await
}

use sqlx::{PgPool, Row};
use std::env;
use crate::domain::entities::DbStatus;
use crate::domain::traits::DatabaseHealthChecker;
use async_trait::async_trait;

pub struct PostgresHealthChecker {
    pool: Option<PgPool>,
}

impl PostgresHealthChecker {
    pub async fn new() -> Self {
        let pool = Self::create_db_pool().await.ok();
        Self { pool }
    }

    async fn create_db_pool() -> Result<PgPool, sqlx::Error> {
        let host = env::var("PGSQL_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = env::var("PGSQL_PORT").unwrap_or_else(|_| "5432".to_string());
        let user = env::var("PGSQL_USER").unwrap_or_else(|_| "postgres".to_string());
        let password = env::var("PGSQL_PASSWD").unwrap_or_else(|_| "password".to_string());
        let database = env::var("PGSQL_DB").unwrap_or_else(|_| "test_db".to_string());
        
        let database_url = format!(
            "postgresql://{}:{}@{}:{}/{}?client_encoding=utf8",
            user, password, host, port, database
        );
        
        println!("Подключение к базе данных: postgresql://{}:***@{}:{}/{}", user, host, port, database);
        
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
    }
}

#[async_trait]
impl DatabaseHealthChecker for PostgresHealthChecker {
    async fn check_health(&self) -> DbStatus {
        match &self.pool {
            Some(db_pool) => {
                match sqlx::query("SELECT 1 as test")
                    .fetch_one(db_pool)
                    .await
                {
                    Ok(row) => {
                        let test_value: i32 = row.get("test");
                        if test_value == 1 {
                            DbStatus::connected("База данных доступна".to_string())
                        } else {
                            DbStatus::disconnected("Неверный ответ от базы данных".to_string())
                        }
                    }
                    Err(e) => {
                        DbStatus::disconnected(format!("Ошибка подключения к базе данных: {}", e))
                    }
                }
            }
            None => {
                DbStatus::disconnected("Пул подключений к базе данных не инициализирован".to_string())
            }
        }
    }
}

use sqlx::{PgPool, migrate::MigrateDatabase, Postgres};

/// Функция для запуска миграций базы данных
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    println!("Запуск миграций базы данных...");
    
    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;
    
    println!("Миграции успешно выполнены");
    Ok(())
}

/// Функция для создания базы данных если она не существует
pub async fn ensure_database_exists(database_url: &str) -> Result<(), sqlx::Error> {
    if !Postgres::database_exists(database_url).await.unwrap_or(false) {
        println!("База данных не найдена, создаю новую...");
        Postgres::create_database(database_url).await?;
        println!("База данных создана");
    } else {
        println!("База данных уже существует");
    }
    Ok(())
}

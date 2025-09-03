use crate::domain::entities::{User, CreateUserRequest};
use crate::domain::traits::{UserRepository, UserAuthRepository};
use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use bcrypt::{hash, DEFAULT_COST};

pub struct PostgreSQLUserRepository {
    pool: PgPool,
}

impl PostgreSQLUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Метод для поиска пользователя по username (для аутентификации)
    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, String> {
        let result = sqlx::query(
            r#"
            SELECT id, username, first_name, last_name, email, user_type_id, 
                   created_at, updated_at, deleted_at
            FROM users 
            WHERE username = $1 AND deleted_at IS NULL
            "#
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Ошибка поиска пользователя по username: {}", e))?;

        match result {
            Some(row) => Ok(Some(User {
                id: row.get("id"),
                username: row.get("username"),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                email: row.get("email"),
                user_type_id: row.get("user_type_id"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                deleted_at: row.get("deleted_at"),
            })),
            None => Ok(None),
        }
    }

    /// Метод для получения хеша пароля пользователя (для аутентификации)
    pub async fn get_password_hash(&self, username: &str) -> Result<Option<String>, String> {
        let result = sqlx::query(
            r#"
            SELECT password_hash
            FROM users 
            WHERE username = $1 AND deleted_at IS NULL
            "#
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Ошибка получения пароля: {}", e))?;

        Ok(result.map(|row| row.get("password_hash")))
    }

    /// Создать пользователя с хешированием пароля
    pub async fn create_user_with_password(
        &self, 
        username: &str,
        password: &str,
        first_name: Option<&str>,
        last_name: Option<&str>,
        email: Option<&str>,
        user_type_id: Option<i32>,
    ) -> Result<User, String> {
        // Хешируем пароль
        let password_hash = hash(password.as_bytes(), DEFAULT_COST)
            .map_err(|e| format!("Ошибка хеширования пароля: {}", e))?;

        let result = sqlx::query(
            r#"
            INSERT INTO users (username, password_hash, first_name, last_name, email, user_type_id)
            VALUES ($1, $2, $3, $4, $5, COALESCE($6, 1))
            RETURNING id, username, first_name, last_name, email, user_type_id, 
                      created_at, updated_at, deleted_at
            "#
        )
        .bind(username)
        .bind(password_hash)
        .bind(first_name)
        .bind(last_name)
        .bind(email)
        .bind(user_type_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| format!("Ошибка создания пользователя: {}", e))?;

        Ok(User {
            id: result.get("id"),
            username: result.get("username"),
            first_name: result.get("first_name"),
            last_name: result.get("last_name"),
            email: result.get("email"),
            user_type_id: result.get("user_type_id"),
            created_at: result.get("created_at"),
            updated_at: result.get("updated_at"),
            deleted_at: result.get("deleted_at"),
        })
    }

    /// Обновить пользователя
    pub async fn update_user_fields(
        &self,
        id: Uuid,
        first_name: Option<&str>,
        last_name: Option<&str>,
        email: Option<&str>,
    ) -> Result<Option<User>, String> {
        let result = sqlx::query(
            r#"
            UPDATE users 
            SET first_name = COALESCE($2, first_name),
                last_name = COALESCE($3, last_name),
                email = COALESCE($4, email),
                updated_at = NOW()
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING id, username, first_name, last_name, email, user_type_id, 
                      created_at, updated_at, deleted_at
            "#
        )
        .bind(id)
        .bind(first_name)
        .bind(last_name)
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Ошибка обновления пользователя: {}", e))?;

        match result {
            Some(row) => Ok(Some(User {
                id: row.get("id"),
                username: row.get("username"),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                email: row.get("email"),
                user_type_id: row.get("user_type_id"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                deleted_at: row.get("deleted_at"),
            })),
            None => Ok(None),
        }
    }
}

#[async_trait]
impl UserRepository for PostgreSQLUserRepository {
    async fn create_user(&self, user_data: CreateUserRequest) -> Result<User, String> {
        let result = sqlx::query(
            r#"
            INSERT INTO users (username, password_hash, email, user_type_id)
            VALUES ($1, '', $2, 1)
            RETURNING id, username, first_name, last_name, email, user_type_id, 
                      created_at, updated_at, deleted_at
            "#
        )
        .bind(&user_data.username)
        .bind(&user_data.email)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| format!("Ошибка создания пользователя: {}", e))?;

        Ok(User {
            id: result.get("id"),
            username: result.get("username"),
            first_name: result.get("first_name"),
            last_name: result.get("last_name"),
            email: result.get("email"),
            user_type_id: result.get("user_type_id"),
            created_at: result.get("created_at"),
            updated_at: result.get("updated_at"),
            deleted_at: result.get("deleted_at"),
        })
    }

    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>, String> {
        let result = sqlx::query(
            r#"
            SELECT id, username, first_name, last_name, email, user_type_id, 
                   created_at, updated_at, deleted_at
            FROM users 
            WHERE id = $1 AND deleted_at IS NULL
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Ошибка поиска пользователя: {}", e))?;

        match result {
            Some(row) => Ok(Some(User {
                id: row.get("id"),
                username: row.get("username"),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                email: row.get("email"),
                user_type_id: row.get("user_type_id"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                deleted_at: row.get("deleted_at"),
            })),
            None => Ok(None),
        }
    }

    async fn get_all_users(&self) -> Result<Vec<User>, String> {
        let rows = sqlx::query(
            r#"
            SELECT id, username, first_name, last_name, email, user_type_id, 
                   created_at, updated_at, deleted_at
            FROM users 
            WHERE deleted_at IS NULL
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Ошибка получения списка пользователей: {}", e))?;

        let users = rows.into_iter()
            .map(|row| User {
                id: row.get("id"),
                username: row.get("username"),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                email: row.get("email"),
                user_type_id: row.get("user_type_id"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                deleted_at: row.get("deleted_at"),
            })
            .collect();

        Ok(users)
    }

    async fn delete_user(&self, id: Uuid) -> Result<bool, String> {
        let result = sqlx::query(
            r#"
            UPDATE users 
            SET deleted_at = NOW(), updated_at = NOW()
            WHERE id = $1 AND deleted_at IS NULL
            "#
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Ошибка удаления пользователя: {}", e))?;

        Ok(result.rows_affected() > 0)
    }
}

#[async_trait]
impl UserAuthRepository for PostgreSQLUserRepository {
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, String> {
        self.find_by_username(username).await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, String> {
        self.get_user_by_id(id).await
    }

    async fn get_password_hash(&self, username: &str) -> Result<Option<String>, String> {
        self.get_password_hash(username).await
    }

    async fn create_user_with_password(
        &self, 
        username: &str,
        password: &str,
        first_name: Option<&str>,
        last_name: Option<&str>,
        email: Option<&str>,
        user_type_id: Option<i32>,
    ) -> Result<User, String> {
        self.create_user_with_password(username, password, first_name, last_name, email, user_type_id).await
    }

    async fn update_user_fields(
        &self,
        id: Uuid,
        first_name: Option<&str>,
        last_name: Option<&str>,
        email: Option<&str>,
    ) -> Result<Option<User>, String> {
        self.update_user_fields(id, first_name, last_name, email).await
    }
}

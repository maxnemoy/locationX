use crate::domain::entities::{DbStatus, PingResponse, CreateUserRequest, CreateUserResponse, User, UsersListResponse};
use crate::domain::traits::{DatabaseHealthChecker, HealthService, UserRepository, UserService};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

pub struct HealthServiceImpl {
    db_checker: Arc<dyn DatabaseHealthChecker + Send + Sync>,
}

impl HealthServiceImpl {
    pub fn new(db_checker: Arc<dyn DatabaseHealthChecker + Send + Sync>) -> Self {
        Self { db_checker }
    }
}

#[async_trait]
impl HealthService for HealthServiceImpl {
    async fn status(&self) -> PingResponse {
        PingResponse::default()
    }

    async fn check_database(&self) -> DbStatus {
        self.db_checker.check_health().await
    }
}

pub struct UserServiceImpl {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
}

impl UserServiceImpl {
    pub fn new(user_repository: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn create_user(&self, user_data: CreateUserRequest) -> Result<CreateUserResponse, String> {
        let user = self.user_repository.create_user(user_data).await?;
        
        // Конвертируем из новой структуры User в старую CreateUserResponse
        // TODO: В будущем лучше обновить CreateUserResponse для соответствия новой User структуре
        Ok(CreateUserResponse {
            id: 1, // Временно используем фиксированное значение для совместимости
            username: user.username,
            email: user.email.unwrap_or_default(),
            message: "Пользователь успешно создан".to_string(),
        })
    }

    async fn get_user(&self, id: Uuid) -> Result<User, String> {
        match self.user_repository.get_user_by_id(id).await? {
            Some(user) => Ok(user),
            None => Err("Пользователь не найден".to_string()),
        }
    }

    async fn get_all_users(&self) -> Result<UsersListResponse, String> {
        let users = self.user_repository.get_all_users().await?;
        let total = users.len();
        Ok(UsersListResponse { users, total })
    }

    async fn delete_user(&self, id: Uuid) -> Result<String, String> {
        if self.user_repository.delete_user(id).await? {
            Ok("Пользователь успешно удален".to_string())
        } else {
            Err("Пользователь не найден".to_string())
        }
    }
}

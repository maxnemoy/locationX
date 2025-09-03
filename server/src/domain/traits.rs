use crate::domain::entities::{DbStatus, User, CreateUserRequest, CreateUserResponse, UsersListResponse};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait DatabaseHealthChecker {
    async fn check_health(&self) -> DbStatus;
}

#[async_trait] 
pub trait HealthService {
    async fn status(&self) -> crate::domain::entities::PingResponse;
    async fn check_database(&self) -> DbStatus;
}

#[async_trait]
pub trait UserRepository {
    async fn create_user(&self, user_data: CreateUserRequest) -> Result<User, String>;
    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>, String>;
    async fn get_all_users(&self) -> Result<Vec<User>, String>;
    async fn delete_user(&self, id: Uuid) -> Result<bool, String>;
}

// Новый trait для аутентификации
#[async_trait]
pub trait UserAuthRepository {
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, String>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, String>;
    async fn get_password_hash(&self, username: &str) -> Result<Option<String>, String>;
    async fn create_user_with_password(
        &self, 
        username: &str,
        password: &str,
        first_name: Option<&str>,
        last_name: Option<&str>,
        email: Option<&str>,
        user_type_id: Option<i32>,
    ) -> Result<User, String>;
    async fn update_user_fields(
        &self,
        id: Uuid,
        first_name: Option<&str>,
        last_name: Option<&str>,
        email: Option<&str>,
    ) -> Result<Option<User>, String>;
}

#[async_trait]
pub trait UserService {
    async fn create_user(&self, user_data: CreateUserRequest) -> Result<CreateUserResponse, String>;
    async fn get_user(&self, id: Uuid) -> Result<User, String>;
    async fn get_all_users(&self) -> Result<UsersListResponse, String>;
    async fn delete_user(&self, id: Uuid) -> Result<String, String>;
}

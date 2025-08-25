use crate::domain::entities::{DbStatus, User, CreateUserRequest, CreateUserResponse, UsersListResponse};
use async_trait::async_trait;

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
    async fn get_user_by_id(&self, id: u32) -> Result<Option<User>, String>;
    async fn get_all_users(&self) -> Result<Vec<User>, String>;
    async fn delete_user(&self, id: u32) -> Result<bool, String>;
}

#[async_trait]
pub trait UserService {
    async fn create_user(&self, user_data: CreateUserRequest) -> Result<CreateUserResponse, String>;
    async fn get_user(&self, id: u32) -> Result<User, String>;
    async fn get_all_users(&self) -> Result<UsersListResponse, String>;
    async fn delete_user(&self, id: u32) -> Result<String, String>;
}

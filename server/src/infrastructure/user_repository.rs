use crate::domain::entities::{User, CreateUserRequest};
use crate::domain::traits::UserRepository;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::Utc;

pub struct InMemoryUserRepository {
    users: Arc<Mutex<HashMap<Uuid, User>>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn create_user(&self, user_data: CreateUserRequest) -> Result<User, String> {
        let mut users = self.users.lock().map_err(|_| "Failed to acquire lock")?;
        
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        let user = User {
            id,
            username: user_data.username,
            first_name: None, // CreateUserRequest не содержит этих полей
            last_name: None,
            email: Some(user_data.email),
            user_type_id: 1, // По умолчанию
            created_at: now,
            updated_at: now,
            deleted_at: None,
        };
        
        users.insert(id, user.clone());
        Ok(user)
    }

    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>, String> {
        let users = self.users.lock().map_err(|_| "Failed to acquire lock")?;
        Ok(users.get(&id).cloned())
    }

    async fn get_all_users(&self) -> Result<Vec<User>, String> {
        let users = self.users.lock().map_err(|_| "Failed to acquire lock")?;
        Ok(users.values().cloned().collect())
    }

    async fn delete_user(&self, id: Uuid) -> Result<bool, String> {
        let mut users = self.users.lock().map_err(|_| "Failed to acquire lock")?;
        Ok(users.remove(&id).is_some())
    }
}

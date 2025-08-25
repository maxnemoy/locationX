use crate::domain::entities::{User, CreateUserRequest};
use crate::domain::traits::UserRepository;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub struct InMemoryUserRepository {
    users: Arc<Mutex<HashMap<u32, User>>>,
    next_id: Arc<Mutex<u32>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn create_user(&self, user_data: CreateUserRequest) -> Result<User, String> {
        let mut next_id = self.next_id.lock().map_err(|_| "Failed to acquire lock")?;
        let mut users = self.users.lock().map_err(|_| "Failed to acquire lock")?;
        
        let id = *next_id;
        *next_id += 1;
        
        let user = User {
            id,
            username: user_data.username,
            email: user_data.email,
            created_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string(),
        };
        
        users.insert(id, user.clone());
        Ok(user)
    }

    async fn get_user_by_id(&self, id: u32) -> Result<Option<User>, String> {
        let users = self.users.lock().map_err(|_| "Failed to acquire lock")?;
        Ok(users.get(&id).cloned())
    }

    async fn get_all_users(&self) -> Result<Vec<User>, String> {
        let users = self.users.lock().map_err(|_| "Failed to acquire lock")?;
        Ok(users.values().cloned().collect())
    }

    async fn delete_user(&self, id: u32) -> Result<bool, String> {
        let mut users = self.users.lock().map_err(|_| "Failed to acquire lock")?;
        Ok(users.remove(&id).is_some())
    }
}

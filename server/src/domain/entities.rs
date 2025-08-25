use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug, Clone)]
pub struct DbStatus {
    pub status: String,
    pub message: String,
    pub connected: bool,
}

impl DbStatus {
    pub fn connected(message: String) -> Self {
        Self {
            status: "ok".to_string(),
            message,
            connected: true,
        }
    }

    pub fn disconnected(message: String) -> Self {
        Self {
            status: "error".to_string(),
            message,
            connected: false,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct PingResponse {
    pub status: String,
    pub message: String,
}

impl Default for PingResponse {
    fn default() -> Self {
        Self {
            status: "ok".to_string(),
            message: "pong".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
}

#[derive(Serialize, Debug)]
pub struct CreateUserResponse {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct UsersListResponse {
    pub users: Vec<User>,
    pub total: usize,
}

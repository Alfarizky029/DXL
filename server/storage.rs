
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

pub struct Storage {
    users: Mutex<HashMap<i32, User>>,
    current_id: Mutex<i32>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            users: Mutex::new(HashMap::new()),
            current_id: Mutex::new(1),
        }
    }

    pub fn get_user(&self, id: i32) -> Option<User> {
        self.users.lock().unwrap().get(&id).cloned()
    }

    pub fn get_user_by_username(&self, username: &str) -> Option<User> {
        self.users
            .lock()
            .unwrap()
            .values()
            .find(|user| user.username == username)
            .cloned()
    }

    pub fn create_user(&self, username: String, password: String) -> User {
        let mut current_id = self.current_id.lock().unwrap();
        let id = *current_id;
        *current_id += 1;

        let user = User {
            id,
            username,
            password,
        };

        self.users.lock().unwrap().insert(id, user.clone());
        user
    }
}

lazy_static::lazy_static! {
    pub static ref STORAGE: Storage = Storage::new();
}

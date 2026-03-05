use std::sync::{Arc, Mutex};

use domain::user::{UserRepository, entity::User, value_objects::Email};



pub struct InMemoryUserRepository {
    pub users: Arc<Mutex<Vec<User>>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        InMemoryUserRepository {
            users: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait::async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn find_by_id(&self, id: uuid::Uuid) -> Option<User> {
        for user in self.users.lock().unwrap().iter() {
            if user.id == id {
                return Some(user.clone());
            }
        }
        None
    }

    async fn find_by_email(&self, email: &Email) -> anyhow::Result<Option<User>> {
        for user in self.users.lock().unwrap().iter() {
            if &user.email == email {
                return Ok(Some(user.clone()));
            }
        }
        Ok(None)
    }

    async fn save(&self, user: &User) -> anyhow::Result<User> {
        let new_user = user.clone();
        self.users.lock().unwrap().push(new_user.clone());
        Ok(new_user)
    }
}
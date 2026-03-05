use std::sync::{Arc, Mutex};

use domain::user::{UserOAuthRepository, entity::User, value_objects::OAuthProvider};

pub struct InMemoryOAuthRepository {
    pub users: Arc<Mutex<Vec<User>>>,
}

impl InMemoryOAuthRepository {
    pub fn new() -> Self {
        InMemoryOAuthRepository {
            users: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait::async_trait]
impl UserOAuthRepository for InMemoryOAuthRepository {
    async fn find_by_provider(
        &self,
        provider: &OAuthProvider,
        provider_user_id: &str,
    ) -> anyhow::Result<Option<User>> {
        let _provider = provider;
        let _provider_user_id = provider_user_id;
        for user in self.users.lock().unwrap().iter() {
            let _ = user;
        }
        Ok(None) // In a real implementation, you would search for a user with the given provider and provider_user_id
    }

    async fn attach_provider(
        &self,
        user_id: uuid::Uuid,
        provider: &OAuthProvider,
        provider_user_id: &str,
    ) -> anyhow::Result<()> {
        let _user_id = user_id;
        let _provider = provider;
        let _provider_user_id = provider_user_id;
        Ok(()) // In a real implementation, you would find the user by user_id and attach the provider information to that user
    }
}
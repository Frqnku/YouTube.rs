use uuid::Uuid;

use crate::user::{
    entity::User,
    value_objects::{Email, OAuthProvider},
};

#[async_trait::async_trait]
pub trait UserRepository {
    async fn find_by_id(&self, id: Uuid) -> Option<User>;
    async fn find_by_email(&self, email: &Email) -> anyhow::Result<Option<User>>;
    async fn save(&self, user: &User) -> anyhow::Result<User>;
}

#[async_trait::async_trait]
pub trait UserOAuthRepository {
    async fn find_by_provider(
        &self,
        provider: &OAuthProvider,
        provider_user_id: &str,
    ) -> anyhow::Result<Option<User>>;

    async fn attach_provider(
        &self,
        user_id: Uuid,
        provider: &OAuthProvider,
        provider_user_id: &str,
    ) -> anyhow::Result<()>;
}
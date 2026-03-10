use domain::{_shared::value_objects::Url, user::{UserOAuthRepository, UserRepository, entity::User, value_objects::{Email, OAuthProvider}}};
use sqlx::types::Uuid;

pub struct PgUserRepository {
    pool: sqlx::PgPool,
}

impl PgUserRepository {
    pub fn new(pool: &sqlx::PgPool) -> Self {
        Self {
            pool: pool.clone(),
        }
    }
}

struct UserRecord {
    id: Uuid,
    name: String,
    email: String,
    profile_picture: Option<String>,
}

impl UserRecord {
    fn into_user(self) -> anyhow::Result<User> {
        let profile_picture = self
            .profile_picture
            .map(Url::try_from)
            .transpose()?;

        Ok(User::new(
            self.id,
            self.name,
            Email::try_from(self.email)?,
            profile_picture,
        ))
    }
}

#[async_trait::async_trait]
impl UserRepository for PgUserRepository {
    async fn find_by_id(&self, id: Uuid) -> Option<User> {
        let record = sqlx::query_as!(
            UserRecord,
            "SELECT id, name, email, profile_picture
            FROM users
            WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
        .ok()?;

        if let Some(rec) = record {
            Some(rec.into_user().ok()?)
        } else {
            None
        }
    }

    async fn find_by_email(&self, email: &Email) -> anyhow::Result<Option<User>> {
        let record = sqlx::query_as!(
            UserRecord,
            "SELECT id, name, email, profile_picture
            FROM users
            WHERE email = $1",
            email.as_str()
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(rec) = record {
            Ok(Some(rec.into_user()?))
        } else {
            Ok(None)
        }
    }

    async fn save(&self, user: &User) -> anyhow::Result<User> {
        let record = sqlx::query_as!(
            UserRecord,
            "INSERT INTO users (name, email, profile_picture)
            VALUES ($1, $2, $3)
            RETURNING id, name, email, profile_picture",
            user.name.as_str(),
            user.email.as_str(),
            user.profile_picture.as_ref().map(Url::as_str),
        )
        .fetch_one(&self.pool)
        .await?;

        let saved_user = record.into_user()?;
        Ok(saved_user)
    }
}

#[async_trait::async_trait]
impl UserOAuthRepository for PgUserRepository {
    async fn find_by_provider(
        &self,
        provider: &OAuthProvider,
        provider_user_id: &str,
    ) -> anyhow::Result<Option<User>> {
        let record = sqlx::query_as!(
            UserRecord,
            "SELECT u.id, u.name, u.email, u.profile_picture
            FROM users u
            JOIN user_oauth_providers co ON u.id = co.user_id
            WHERE co.provider = $1 AND co.provider_user_id = $2",
            provider.as_str(),
            provider_user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(rec) = record {
            Ok(Some(rec.into_user()?))
        } else {
            Ok(None)
        }
    }

    async fn attach_provider(
        &self,
        user_id: Uuid,
        provider: &OAuthProvider,
        provider_user_id: &str,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            "INSERT INTO user_oauth_providers (user_id, provider, provider_user_id)
            VALUES ($1, $2, $3)",
            user_id,
            provider.as_str(),
            provider_user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
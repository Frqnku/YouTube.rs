use domain::{_shared::DomainError, user::{UserOAuthRepository, UserRepository, entity::User}};
use uuid::Uuid;

use crate::{dtos::OAuthUserDto, services::TokenService};

pub struct AuthenticateWithOAuth<
    'a,
    R: UserRepository,
    A: UserOAuthRepository,
    T: TokenService,
> {
    pub repository: &'a R,
    pub oauth_repository: &'a A,
    pub token_service: &'a T,
}

impl<'a, R, A, T> AuthenticateWithOAuth<'a, R, A, T>
where
    R: UserRepository,
    A: UserOAuthRepository,
    T: TokenService,
{
    pub async fn execute(
        &self,
        oauth_user: OAuthUserDto,
    ) -> anyhow::Result<String, DomainError> {

        // 1️⃣ Strict OAuth authentication:
        // Try to authenticate using (provider, provider_user_id)
        if let Some(user) = self
            .oauth_repository
            .find_by_provider(
                &oauth_user.provider,
                &oauth_user.provider_user_id,
            )
            .await?
        {
            // Provider already linked → successful authentication
            return Ok(
                self.token_service
                    .generate_token(&user.id.to_string(), &user.name, user.profile_picture)?
            );
        }

        // 2️⃣ No provider match found.
        // Check if a user already exists with the same email.
        if let Some(user) = self
            .repository
            .find_by_email(&oauth_user.email)
            .await?
        {
            // 3️⃣ OAuth-only account found:
            // Attach the new OAuth provider to this user
            self.oauth_repository
                .attach_provider(
                    user.id,
                    &oauth_user.provider,
                    &oauth_user.provider_user_id,
                )
                .await?;

            return Ok(
                self.token_service
                    .generate_token(&user.id.to_string(), &user.name, user.profile_picture)?
            );
        }

        // 4️⃣ No user found:
        // Create a new OAuth-only user
        // verified by default since OAuth providers verify emails
        let new_user = User::new(
            Uuid::new_v4(),
            oauth_user.name,
            oauth_user.email,
            oauth_user.picture,
        );

        let saved_user = self.repository.save(&new_user).await?;

        // 5️⃣ Link the OAuth provider to the newly created user
        self.oauth_repository
            .attach_provider(
                saved_user.id,
                &oauth_user.provider,
                &oauth_user.provider_user_id,
            )
            .await?;

        // 6️⃣ Generate token
        let token = self.token_service.generate_token(&saved_user.id.to_string(), &saved_user.name, saved_user.profile_picture)?;

        Ok(token)
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use domain::user::value_objects::{Email, OAuthProvider};

    use crate::_tests::{repositories::{InMemoryUserRepository, InMemoryOAuthRepository}, services::MockTokenService};

    #[tokio::test]
    async fn test_authenticate_with_oauth_new_user() {
        let user_repo = InMemoryUserRepository::new();
        let oauth_repo = InMemoryOAuthRepository::new();
        let token_service = MockTokenService::new("test_secret".to_string());

        let command = AuthenticateWithOAuth {
            repository: &user_repo,
            oauth_repository: &oauth_repo,
            token_service: &token_service,
        };

        let dto = OAuthUserDto {
            provider: OAuthProvider::Google,
            provider_user_id: "google-123".to_string(),
            email: Email::try_from("new.user@example.com").unwrap(),
            name: "New".to_string(),
            picture: None,
        };

        let result = command.execute(dto).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authenticate_with_oauth_existing_user() {
        let user_repo = InMemoryUserRepository::new();
        let oauth_repo = InMemoryOAuthRepository::new();
        let token_service = MockTokenService::new("test_secret".to_string());

        let command = AuthenticateWithOAuth {
            repository: &user_repo,
            oauth_repository: &oauth_repo,
            token_service: &token_service,
        };

        let email = Email::try_from("existing.user@example.com").unwrap();
        let existing_user = User::new(
            Uuid::new_v4(),
            "ExistingUser".to_string(),
            email.clone(),
            None,
        );
        user_repo.save(&existing_user).await.unwrap();

        let dto = OAuthUserDto {
            provider: OAuthProvider::Google,
            provider_user_id: "google-123".to_string(),
            email,
            name: "Existing".to_string(),
            picture: None,
        };

        let result = command.execute(dto).await;
        assert!(result.is_ok());
    }
}
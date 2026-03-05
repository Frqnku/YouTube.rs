use crate::dtos::OAuthUserDto;

#[async_trait::async_trait]
pub trait OAuthService: Send + Sync {
    async fn authenticate(
        &self,
        code: String,
        redirect_uri: String,
    ) -> anyhow::Result<OAuthUserDto>;
}
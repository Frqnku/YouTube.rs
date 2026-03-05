use std::borrow::Cow;

use application::{dtos::OAuthUserDto, services::OAuthService};
use domain::user::value_objects::{Email, OAuthProvider};
use oauth2::{AuthUrl, AuthorizationCode, ClientId, ClientSecret, RedirectUrl, TokenResponse, TokenUrl, basic::BasicClient};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GoogleProfile {
    sub: String,
    email: String,
    name: String,
    picture: String,
}

pub struct GoogleOAuthService {
    client: BasicClient,
    http: reqwest::Client,
}

impl GoogleOAuthService {
    pub fn new(
        client_id: String,
        client_secret: String,
        redirect_uri: String,
    ) -> Self {
        let client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
                .expect("Invalid auth URL"),
            Some(
                TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
                    .expect("Invalid token URL"),
            ),
        )
        .set_redirect_uri(
            RedirectUrl::new(redirect_uri).expect("Invalid redirect URI"),
        );

        Self {
            client,
            http: reqwest::Client::new(),
        }
    }

    async fn fetch_google_profile(
        &self,
        access_token: &str,
    ) -> anyhow::Result<GoogleProfile> {

        let res = self.http
            .get("https://openidconnect.googleapis.com/v1/userinfo")
            .bearer_auth(access_token)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json::<GoogleProfile>().await?)
    }
}

#[async_trait::async_trait]
impl OAuthService for GoogleOAuthService {
    async fn authenticate(
        &self,
        code: String,
        redirect_uri: String,
    ) -> anyhow::Result<OAuthUserDto> {
        let redirect_url = RedirectUrl::new(redirect_uri)?;
        let token = self.client
            .exchange_code(AuthorizationCode::new(code))
            .set_redirect_uri(Cow::Borrowed(&redirect_url))
            .request_async(oauth2::reqwest::async_http_client)
            .await?;

        let profile = self.fetch_google_profile(
            token.access_token().secret()
        ).await?;

        Ok(OAuthUserDto::new(
            OAuthProvider::Google,
            profile.sub,
            Email::try_from(profile.email)?,
            profile.name,
            Some(profile.picture)
        ))
    }
}

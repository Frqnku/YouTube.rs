use std::env;

use leptos::prelude::*;
use urlencoding::encode;
#[cfg(feature = "ssr")]
use domain::user::value_objects::OAuthProvider;
#[cfg(feature = "ssr")]
use application::{commands::AuthenticateWithOAuth, dtos::OAuthUserDto, services::OAuthService};
#[cfg(feature = "ssr")]
use infrastructure::{repositories::PgUserRepository, services::{JwtService, GoogleOAuthService}};

use crate::api::{_errors::{AppServerError, OptionExt, ValidateExt}, _helpers::{Cookie, CookieOptions}};

#[server]
pub async fn oauth(
    provider: String,
    code: Option<String>,
    cookie_state: Option<String>,
    oauth_state: Option<String>
) -> Result<String, AppServerError> {
    let (code, cookie_state, oauth_state) = match (code, cookie_state, oauth_state) {
        (Some(c), Some(cs), Some(os)) => (c, cs, os),
        _ => return Err(AppServerError::new("missing_oauth_parameters", "Missing OAuth parameters".to_string())),
    };

    // 1️⃣ Convert provider string → enum
    let (provider, client_id, client_secret, redirect_uri) = match provider.to_lowercase().as_str() {
        "google" => (OAuthProvider::Google,
            env::var("GOOGLE_CLIENT_ID").map_err(|_| AppServerError::new("missing_google_client_id", "GOOGLE_CLIENT_ID not set".to_string()))?,
            env::var("GOOGLE_CLIENT_SECRET").map_err(|_| AppServerError::new("missing_google_client_secret", "GOOGLE_CLIENT_SECRET not set".to_string()))?,
            env::var("OAUTH_GOOGLE_REDIRECT_URI").map_err(|_| AppServerError::new("missing_google_redirect_uri", "OAUTH_GOOGLE_REDIRECT_URI not set".to_string()))?,
        ),
        other => return Err(AppServerError::new("invalid_provider", format!("Unsupported OAuth provider: {}", other))),
    };

    // 2️⃣ Validate state parameter against cookie
    let csrf = oauth_state.split("|").next().unwrap_or_default();
    if cookie_state != csrf {
        return Err(AppServerError::new("invalid_oauth_state", "OAuth state parameter does not match cookie".to_string()));
    }
    let redirect_to = oauth_state.split("|").nth(1);

    // 3️⃣ Call infra OAuth service to get OAuthUserDto
    let oauth_service: Box<dyn OAuthService> = match provider {
        OAuthProvider::Google => {
            Box::new(GoogleOAuthService::new(
                client_id,
                client_secret,
                redirect_uri.clone(),
            ))
        },
    };

    let oauth_user_dto: OAuthUserDto = oauth_service
        .authenticate(code, redirect_uri)
        .await?;

    // 4️⃣ Validate the DTO
    oauth_user_dto.validate_or_error()?;

    // 5️⃣ Instantiate repositories + token provider
    let pool = use_context::<sqlx::PgPool>()
        .require_context("Missing pool")?;
    let jwt_secret = use_context::<String>()
        .require_context("Missing JWT secret")?;

    let repository = PgUserRepository::new(&pool);
    let token_service = JwtService::new(jwt_secret);

    // 6️⃣ Call use case
    let command = AuthenticateWithOAuth {
        repository: &repository,
        oauth_repository: &repository,
        token_service: &token_service,
    };

    let token = command
        .execute(oauth_user_dto)
        .await?;

    // 7️⃣ Save token in cookies
    let response = use_context::<leptos_axum::ResponseOptions>()
        .ok_or_else(|| AppServerError::new("internal", "Missing ResponseOptions"))?;
    Cookie::new("token", &token)
        .with_options(vec![
            CookieOptions::HttpOnly,
            CookieOptions::Path("/".to_string()),
            CookieOptions::SameSiteLax,
        ])
        .into_response_headers(&response);

    // 8️⃣ Ok
    Ok(redirect_to.unwrap_or("/").to_string())
}

#[server]
pub async fn generate_oauth_url(
    provider: String,
    redirect_to: String,
) -> Result<String, AppServerError> {
    let provider = match provider.to_lowercase().as_str() {
        "google" => OAuthProvider::Google,
        other => return Err(AppServerError::new("invalid_provider", format!("Unsupported OAuth provider: {}", other))),
    };

    let (client_id, redirect_uri) = match provider {
        OAuthProvider::Google => (
            env::var("GOOGLE_CLIENT_ID")
                .map_err(|_| AppServerError::new("missing_google_client_id", "GOOGLE_CLIENT_ID not set".to_string()))?,
            env::var("OAUTH_GOOGLE_REDIRECT_URI")
                .map_err(|_| AppServerError::new("missing_google_redirect_uri", "OAUTH_GOOGLE_REDIRECT_URI not set".to_string()))?,
        ),
    };

    use rand::{distributions::Alphanumeric, Rng};
    let csrf = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect::<String>();

    let response = use_context::<leptos_axum::ResponseOptions>()
        .ok_or_else(|| AppServerError::new("internal", "Missing ResponseOptions"))?;
    Cookie::new("oauth_state", &csrf)
        .with_options(vec![
            CookieOptions::Path("/".to_string()),
            CookieOptions::SameSiteLax,
        ])
        .into_response_headers(&response);

    let state = format!("{}|{}", csrf, redirect_to);
    let state = encode(&state);
    let redirect_uri = encode(&redirect_uri);

    let oauth_url = match provider {
        OAuthProvider::Google => format!(
            "https://accounts.google.com/o/oauth2/v2/auth\
            ?response_type=code\
            &client_id={client_id}\
            &redirect_uri={redirect_uri}\
            &scope=openid%20email%20profile\
            &state={state}\
            &access_type=offline\
            &prompt=consent"
        ),
    };

    Ok(oauth_url)
}

#[server]
pub async fn logout() -> Result<(), AppServerError> {
    let response = use_context::<leptos_axum::ResponseOptions>()
        .ok_or_else(|| AppServerError::new("internal", "Missing ResponseOptions"))?;
    Cookie::new("token", "")
        .with_options(vec![
            CookieOptions::HttpOnly,
            CookieOptions::Path("/".to_string()),
            CookieOptions::SameSiteLax,
            CookieOptions::MaxAge(0),
        ])
        .into_response_headers(&response);

    Ok(())
}
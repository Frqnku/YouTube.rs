use axum::extract::FromRef;
use leptos::prelude::LeptosOptions;
use leptos::prelude::*;
use sqlx::postgres::PgPool;

#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pool: PgPool,
    pub jwt_secret: String,
}

pub fn use_app_state() -> Result<AppState, ServerFnError> {
    use_context::<AppState>().ok_or_else(|| ServerFnError::ServerError("App state missing.".into()))
}

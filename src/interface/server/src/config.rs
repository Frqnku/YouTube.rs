use std::env;

use web::state::AppState;
use leptos::config::LeptosOptions;
use sqlx::postgres::PgPoolOptions;

pub async fn configure_app_state(leptos_options: LeptosOptions) -> anyhow::Result<AppState> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET not set");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap_or_else(|_| {
            panic!("Failed to create Postgres connection pool! URL: {database_url}")
        });

    Ok(AppState {
        leptos_options,
        pool,
        jwt_secret,
    })
}
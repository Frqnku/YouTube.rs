#![recursion_limit = "256"]

use dotenv::dotenv;
use leptos::prelude::*;
use tracing::info;
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

// pub mod fallback;
// pub use fallback::*;
pub mod router;
pub use router::*;
pub mod config;
pub use config::*;
pub mod middleware;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    LogTracer::init().expect("Failed to set logger");

    let subscriber = FmtSubscriber::builder()
        .with_ansi(true)
        .with_file(true)
        .with_line_number(true)
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Could not set subscriber");
    dotenv().ok();

    let conf = get_configuration(None)?;
    let addr = conf.leptos_options.site_addr;

    let app_state = configure_app_state(conf.leptos_options).await?;

    info!("Running database migrations...");
    sqlx::migrate!("../../../migrations").run(&app_state.pool).await?;
    info!("Database migrations completed successfully! ✔️");

    let app = build_app_router(app_state).await?;
    info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

use axum::{
    body::Body as AxumBody,
    extract::State,
    http::Request,
    middleware,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use leptos::prelude::*;
use leptos_axum::{
    LeptosRoutes, generate_route_list, handle_server_fns_with_context, render_app_to_stream_with_context
};
use tower_http::services::ServeDir;

use web::{app::{App, ClientRequestMeta, CurrentUser}, shell::shell, state::AppState};

use crate::middleware::{get_current_ip, get_current_user, require_auth};

/* ========================================================== */
/*                         🦀 MAIN 🦀                        */
/* ========================================================== */

pub async fn build_app_router(
    app_state: AppState,
) -> anyhow::Result<Router> {
    let routes = generate_route_list(|| view! { <App /> });

    Ok(Router::new()
        .nest_service("/pkg", ServeDir::new("target/site/pkg"))
        .nest_service("/assets", ServeDir::new("target/site/assets"))
        .nest_service("/videos", ServeDir::new("videos"))
        .route(
            "/api/{*fn_name}",
            get(server_fn_handler).post(server_fn_handler),
        )
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .layer(middleware::from_fn(require_auth))
        .layer(middleware::from_fn(get_current_user))
        .layer(middleware::from_fn(get_current_ip))
        .with_state(app_state))
}

/* ========================================================== */
/*             ✨ SERVER FUNCTIONS HANDLERS ✨               */
/* ========================================================== */

#[axum_macros::debug_handler]
pub async fn server_fn_handler(
    State(state): State<AppState>,
    req: Request<AxumBody>,
) -> impl IntoResponse {
    let user = req
        .extensions()
        .get::<CurrentUser>()
        .cloned();

    let client_meta = req
        .extensions()
        .get::<ClientRequestMeta>()
        .cloned();

    handle_server_fns_with_context(
        move || {
            provide_context(state.pool.clone());
            provide_context(state.jwt_secret.clone());
            if let Some(user) = user.clone() {
                provide_context(user);
            }
            if let Some(client_meta) = client_meta.clone() {
                provide_context(client_meta);
            }
        },
        req,
    )
    .await
}

#[axum_macros::debug_handler]
pub async fn leptos_routes_handler(
    State(app_state): State<AppState>,
    req: Request<AxumBody>,
) -> Response {
    let leptos_options = app_state.leptos_options.clone();

    let user = req
        .extensions()
        .get::<CurrentUser>()
        .cloned();

    let client_meta = req
        .extensions()
        .get::<ClientRequestMeta>()
        .cloned();

    let handler = render_app_to_stream_with_context(
        move || {
            provide_context(app_state.pool.clone());
            provide_context(app_state.jwt_secret.clone());
            if let Some(user) = user.clone() {
                provide_context(user);
            }
            if let Some(client_meta) = client_meta.clone() {
                provide_context(client_meta);
            }
        },
        move || shell(leptos_options.clone()),
    );

    handler(req).await.into_response()
}

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

use web::{app::App, context::{ClientRequestMeta, CurrentUser, ProvideContextExt, state::AppState}, shell::shell};

use crate::middleware::{get_current_ip, get_current_user};

/* ========================================================== */
/*                         🦀 MAIN 🦀                        */
/* ========================================================== */

pub async fn build_app_router(
    app_state: AppState,
) -> anyhow::Result<Router> {
    let routes = generate_route_list(|| view! { <App /> });

    Ok(Router::new()
        .nest_service("/pkg", ServeDir::new("/site/pkg"))
        .nest_service("/assets", ServeDir::new("/site/assets"))
        .nest_service("/videos", ServeDir::new("videos"))
        .route(
            "/api/{*fn_name}",
            get(server_fn_handler).post(server_fn_handler),
        )
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .layer(middleware::from_fn(get_current_user))
        .layer(middleware::from_fn(get_current_ip))
        .with_state(app_state))
}

/* ========================================================== */
/*             ✨ SERVER FUNCTIONS HANDLERS ✨               */
/* ========================================================== */

#[derive(Clone)]
struct RequestContext {
    state: AppState,
    user: Option<CurrentUser>,
    client_meta: Option<ClientRequestMeta>,
}

impl RequestContext {
    fn new(state: AppState, req: &Request<AxumBody>) -> Self {
        let user = req
            .extensions()
            .get::<CurrentUser>()
            .cloned();

        let client_meta = req
            .extensions()
            .get::<ClientRequestMeta>()
            .cloned();

        Self {
            state,
            user,
            client_meta,
        }
    }
}

impl ProvideContextExt for RequestContext {
    fn provide_contexts(&self) {
        provide_context(self.state.pool.clone());
        provide_context(self.state.jwt_secret.clone());

        if let Some(user) = &self.user {
            provide_context(user.clone());
        }

        if let Some(meta) = &self.client_meta {
            provide_context(meta.clone());
        }
    }
}

#[axum_macros::debug_handler]
pub async fn server_fn_handler(
    State(state): State<AppState>,
    req: Request<AxumBody>,
) -> impl IntoResponse {
    let ctx = RequestContext::new(state, &req);

    handle_server_fns_with_context(
        move || {
            ctx.provide_contexts();
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

    let ctx = RequestContext::new(app_state, &req);

    let handler = render_app_to_stream_with_context(
        move || {
            ctx.provide_contexts();
        },
        move || shell(leptos_options.clone()),
    );

    handler(req).await.into_response()
}
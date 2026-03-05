use std::env;

use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::cookie::CookieJar;
use application::services::TokenService;
use infrastructure::services::JwtService;
use interface::app::CurrentUser;

pub async fn get_current_user(
    mut req: Request,
    next: Next,
) -> Response {
    let jar = CookieJar::from_headers(req.headers());
    let jwt_secret = env::var("JWT_SECRET").unwrap();
    if let Some(cookie) = jar.get("token") {
        let token_provider = JwtService::new(jwt_secret);
        if let Ok(current_user) = token_provider.verify_token(cookie.value()) {
                req.extensions_mut().insert(
                    CurrentUser {
                        name: current_user.name,
                        profile_picture: current_user.profile_picture,
                    }
                );
        }
    }

    next.run(req).await
}

pub async fn require_auth(
    req: Request,
    next: Next,
) -> Response {
    const AUTH_PROTECTED_PATHS: [&str; 2] = ["/history", "/liked-videos"];

    let path = req.uri().path();
    if !AUTH_PROTECTED_PATHS.contains(&path) {
        return next.run(req).await;
    }

    let current_user = req
        .extensions()
        .get::<CurrentUser>();

    if current_user.is_none() {
        return Redirect::to("/").into_response();
    }

    next.run(req).await
}
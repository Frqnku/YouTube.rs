use std::env;
use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, Request},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::cookie::CookieJar;
use application::services::TokenService;
use infrastructure::services::JwtService;
use web::app::{ClientRequestMeta, CurrentUser};

fn extract_client_ip(req: &Request) -> Option<String> {
    let forwarded = req
        .headers()
        .get("x-forwarded-for")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.split(',').next())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned);

    if forwarded.is_some() {
        return forwarded;
    }

    let real_ip = req.headers()
        .get("x-real-ip")
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned);

    if real_ip.is_some() {
        return real_ip;
    }

    req.extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|connect_info| connect_info.0.ip().to_string())
}

pub async fn get_current_ip(
    mut req: Request,
    next: Next,
) -> Response {
    let client_ip = extract_client_ip(&req);
    req.extensions_mut().insert(ClientRequestMeta {
        ip_address: client_ip,
    });

    next.run(req).await
}

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
                        id: current_user.id.to_string(),
                        name: current_user.name,
                        profile_picture: current_user.profile_picture.map(|url| url.to_string()),
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
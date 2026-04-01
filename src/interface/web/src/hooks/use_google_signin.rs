use leptos::prelude::*;

use crate::api::user::auth::GenerateOauthUrl;

pub fn use_google_signin() -> Callback<()> {
    let generate_oauth_url_action = ServerAction::<GenerateOauthUrl>::new();

    Effect::new(move || {
        if let Some(Ok(oauth_url)) = generate_oauth_url_action.value().get() {
            if let Some(window) = web_sys::window() {
                let _ = window.location().set_href(&oauth_url);
            }
        }
    });

    Callback::new(move |_| {
        let redirect_to = web_sys::window()
            .and_then(|window| {
                let location = window.location();
                location
                    .pathname()
                    .ok()
                    .map(|path| if path == "/signin" { "/".to_string() } else { path })
            })
            .unwrap_or_else(|| "/".to_string());

        generate_oauth_url_action.dispatch(GenerateOauthUrl {
            provider: "google".to_string(),
            redirect_to,
        });
    })
}
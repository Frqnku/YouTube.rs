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
                let path = location.pathname().ok()?;
                if path == "/signin" {
                    return Some("/".to_string());
                }

                let search = location.search().ok().unwrap_or_default();
                let hash = location.hash().ok().unwrap_or_default();
                Some(format!("{path}{search}{hash}"))
            })
            .unwrap_or_else(|| "/".to_string());

        generate_oauth_url_action.dispatch(GenerateOauthUrl {
            provider: "google".to_string(),
            redirect_to,
        });
    })
}
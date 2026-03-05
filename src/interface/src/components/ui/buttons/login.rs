use leptos::prelude::*;

use crate::api::user::auth::GenerateOauthUrl;

#[component]
pub fn LoginButton() -> impl IntoView {
    let generate_oauth_url_action = ServerAction::<GenerateOauthUrl>::new();

    let handle_login = move |_| {
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
    };

    Effect::new(move || {
        if let Some(Ok(oauth_url)) = generate_oauth_url_action.value().get() {
            web_sys::window()
                .unwrap()
                .location()
                .set_href(&oauth_url)
                .unwrap();
        }
    });

    view! {
        <div class="w-full">
            <button
                type="button"
                class="btn-secondary text-xs md:text-sm"
                on:click=handle_login
            >
                Sign in
            </button>
        </div>
    }
}
use leptos::{prelude::*, server::codee::string::FromToStringCodec};
use leptos_router::hooks::use_query_map;
use leptos_use::use_cookie;

use crate::{api::user::auth::Oauth, components::ui::Loader};

#[derive(Clone, Copy)]
enum OAuthState {
    Pending,
    Success,
    Error,
}

#[component]
pub fn SigninPage() -> impl IntoView {
    let query_map = use_query_map();
    let code = move || query_map.with(|qm| qm.get("code").map(|t| t.to_string()));
    let state_from_oauth = move || query_map.with(|qm| qm.get("state").map(|t| t.to_string()));
    let (state_from_cookie, set_state_cookie) = use_cookie::<String, FromToStringCodec>("oauth_state");
    let (_, set_token) = use_cookie::<String, FromToStringCodec>("token");

    let oauth_action = ServerAction::<Oauth>::new();
    let oauth_state = RwSignal::new(OAuthState::Pending);
    let has_attempted = RwSignal::new(false);

    Effect::new(move |_| {
        if !has_attempted.get() {
            let code_value = code();
            let oauth_state_value = state_from_oauth();
            let cookie_state_value = state_from_cookie.get();

            match (code_value, oauth_state_value, cookie_state_value) {
                (Some(code), Some(oauth_state_value), Some(cookie_state_value)) => {
                    has_attempted.set(true);
                    oauth_action.dispatch(Oauth {
                        provider: "google".to_string(),
                        code: Some(code),
                        oauth_state: Some(oauth_state_value),
                        cookie_state: Some(cookie_state_value),
                    });
                }
                (Some(_), Some(_), None) => {}
                _ => {
                    if let Some(window) = web_sys::window() {
                        let _ = window.location().set_href("/");
                    }
                }
            }
        }
    });

    Effect::new(move |_| {
        if oauth_action.pending().get() {
            oauth_state.set(OAuthState::Pending);
        } else {
            match oauth_action.value().get() {
                Some(Ok(redirect_to)) => {
                    oauth_state.set(OAuthState::Success);
                    set_state_cookie.set(None);
                    if let Some(window) = web_sys::window() {
                        let _ = window.location().set_href(&redirect_to);
                    }
                }
                Some(Err(_)) => {
                    oauth_state.set(OAuthState::Error);
                    set_state_cookie.set(None);
                    set_token.set(None);
                }
                None => {}
            }
        }
    });

    view! {
        <div class="flex min-h-dvh flex-col items-center justify-center bg-bg px-4">
            {move || {
                match oauth_state.get() {
                    OAuthState::Pending | OAuthState::Success => view! { <Loader /> }.into_any(),
                    OAuthState::Error => view! {
                        <p class="text-text-secondary">"An error occurred during authentication, try again."</p>
                    }
                    .into_any(),
                }
            }}
        </div>
    }
}
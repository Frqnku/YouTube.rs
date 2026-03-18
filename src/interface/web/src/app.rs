use leptos::prelude::*;
use leptos_meta::{Meta, provide_meta_context};
use leptos_router::StaticSegment;
use leptos_router::components::{Route, Router, Routes};
use serde::{Deserialize, Serialize};

use crate::components::layout::{header::Header, Sidebar};
use crate::components::ui::NotFound;
use crate::routes::public::{ChannelPage, HomePage, ResultsPage, SigninPage, WatchPage, HistoryPage, LikedVideosPage};
use crate::api::user::auth::{get_current_client_meta, get_current_user};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

#[derive(Clone, Copy)]
pub struct ThemeContext {
    pub mode: RwSignal<ThemeMode>,
    pub set_mode: Callback<ThemeMode>,
}

#[cfg(target_arch = "wasm32")]
const THEME_STORAGE_KEY: &str = "yt-theme";

fn initial_theme_mode() -> ThemeMode {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(mode) = read_persisted_theme_mode() {
            return mode;
        }
    }

    ThemeMode::Light
}

#[cfg(target_arch = "wasm32")]
fn read_persisted_theme_mode() -> Option<ThemeMode> {
    let storage = web_sys::window()
        .and_then(|window| window.local_storage().ok().flatten())?;

    let value = storage.get_item(THEME_STORAGE_KEY).ok().flatten()?;
    match value.as_str() {
        "dark" => Some(ThemeMode::Dark),
        "light" => Some(ThemeMode::Light),
        _ => None,
    }
}

fn persist_theme_mode(mode: ThemeMode) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(storage) = web_sys::window().and_then(|window| window.local_storage().ok().flatten()) {
            let stored_value = if mode == ThemeMode::Dark { "dark" } else { "light" };
            let _ = storage.set_item(THEME_STORAGE_KEY, stored_value);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    let _ = mode;
}

fn apply_theme(mode: ThemeMode) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(document) = web_sys::window().and_then(|window| window.document()) {
            if let Some(root) = document.document_element() {
                let class_name = if mode == ThemeMode::Dark { "dark" } else { "" };
                root.set_class_name(class_name);
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    let _ = mode;
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrentUser {
    pub id: String,
    pub name: String,
    pub profile_picture: Option<String>,
}

#[derive(Clone, Copy)]
pub struct CurrentUserContext {
    pub current_user: RwSignal<Option<CurrentUser>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientRequestMeta {
    pub ip_address: Option<String>,
}

#[derive(Clone, Copy)]
pub struct ClientRequestMetaContext {
    pub client_meta: RwSignal<Option<ClientRequestMeta>>,
}

#[derive(Clone, Copy)]
pub struct SubscriptionsContext {
    pub refetch_trigger: RwSignal<u32>,
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let current_client_signal = RwSignal::new(use_context::<ClientRequestMeta>());
    provide_context(ClientRequestMetaContext {
        client_meta: current_client_signal,
    });

    let current_user_signal = RwSignal::new(use_context::<CurrentUser>());
    provide_context(CurrentUserContext {
        current_user: current_user_signal,
    });

    provide_context(SubscriptionsContext {
        refetch_trigger: RwSignal::new(0u32),
    });

    let theme_mode = RwSignal::new(initial_theme_mode());
    let set_theme_mode = Callback::new(move |mode: ThemeMode| {
        theme_mode.set(mode);
    });
    provide_context(ThemeContext {
        mode: theme_mode,
        set_mode: set_theme_mode,
    });

    let current_client_resource = Resource::new(
        move || (),
        move |_| async move { get_current_client_meta().await.ok().flatten() },
    );

    let current_user_resource = Resource::new(
        move || (),
        move |_| async move { get_current_user().await.ok().flatten() },
    );

    Effect::new(move |_| {
        if let Some(current_user) = current_user_resource.get() {
            current_user_signal.set(current_user);
        }
        if let Some(current_client) = current_client_resource.get() {
            current_client_signal.set(current_client);
        }
    });

    Effect::new(move |_| {
        let current_theme = theme_mode.get();
        apply_theme(current_theme);
        persist_theme_mode(current_theme);
    });

    view! {
        <Meta name="viewport" content="width=device-width, initial-scale=1.0, viewport-fit=cover" />

        <Router>
            <Header />
            <div class="flex min-h-[calc(100dvh-3.5rem)] bg-bg">
                <Sidebar />
                <main class="flex-1">
                    <Routes fallback=|| PageNotFound.into_view()>
                        <Route path=StaticSegment("") view=HomePage />
                        <Route path=StaticSegment("results") view=ResultsPage />
                        <Route path=StaticSegment("channel") view=ChannelPage />
                        <Route path=StaticSegment("signin") view=SigninPage />
                        <Route path=StaticSegment("history") view=HistoryPage />
                        <Route path=StaticSegment("liked-videos") view=LikedVideosPage />
                        <Route path=StaticSegment("watch") view=WatchPage />
                    </Routes>
                </main>
            </div>
        </Router>
    }.into_view()
}

#[component]
fn PageNotFound() -> impl IntoView {
    view! {
        <NotFound message="Page not found".to_string() />
    }
}
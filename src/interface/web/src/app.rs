use leptos::prelude::*;
use leptos_meta::Meta;
use leptos_router::StaticSegment;
use leptos_router::components::{Route, Router, Routes};
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

use crate::components::layout::{header::Header, Sidebar};
use crate::components::ui::NotFound;
use crate::context::{
    resources::setup_user_resources,
    setup_app_contexts,
    theme::setup_theme_effect
};
use crate::routes::public::{ChannelPage, HomePage, ResultsPage, SigninPage, WatchPage, HistoryPage, LikedVideosPage};

#[component]
pub fn App() -> impl IntoView {
    let (current_user_signal, current_client_signal, theme_mode) = setup_app_contexts();
    setup_user_resources(current_user_signal, current_client_signal);
    setup_theme_effect(theme_mode);
    let sidebar_open = RwSignal::new(false);

    Effect::new(move |_| {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(media_query)) = window.match_media("(min-width: 768px)") {
                sidebar_open.set(media_query.matches());

                let sidebar_open = sidebar_open;
                let listener = Closure::wrap(Box::new(move |_: web_sys::Event| {
                    if let Some(window) = web_sys::window() {
                        if let Ok(Some(media_query)) = window.match_media("(min-width: 768px)") {
                            sidebar_open.set(media_query.matches());
                        }
                    }
                }) as Box<dyn Fn(web_sys::Event)>);

                let _ = media_query.add_listener_with_opt_callback(Some(listener.as_ref().unchecked_ref()));
                listener.forget();
            }
        }
    });

    view! {
        <Meta name="viewport" content="width=device-width, initial-scale=1.0, viewport-fit=cover" />

        <Router>
            <Header sidebar_open=sidebar_open />
            <div class="flex min-h-[calc(100dvh-3.5rem)] bg-bg">
                <Sidebar sidebar_open=sidebar_open />
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
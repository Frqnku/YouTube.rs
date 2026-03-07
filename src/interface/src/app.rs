use leptos::prelude::*;
use leptos_meta::{Meta, provide_meta_context};
use leptos_router::StaticSegment;
use leptos_router::components::{Route, Router, Routes};
use serde::{Deserialize, Serialize};

use crate::components::ui::layout::{header::Header, Sidebar};
use crate::routes::protected::{HistoryPage, LikedVideosPage};
use crate::routes::public::{HomePage, SigninPage, WatchPage};
use crate::api::user::auth::get_current_user;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrentUser {
    pub name: String,
    pub profile_picture: Option<String>,
}

#[derive(Clone, Copy)]
pub struct CurrentUserContext {
    pub current_user: RwSignal<Option<CurrentUser>>,
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let current_user_signal = RwSignal::new(use_context::<CurrentUser>());
    provide_context(CurrentUserContext {
        current_user: current_user_signal,
    });

    let current_user_resource = Resource::new(
        move || (),
        move |_| async move { get_current_user().await.ok().flatten() },
    );

    Effect::new(move |_| {
        if let Some(current_user) = current_user_resource.get() {
            current_user_signal.set(current_user);
        }
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
        <div class="flex min-h-dvh flex-col items-center justify-center bg-bg px-4">
            <h1 class="text-4xl font-bold text-text">"404"</h1>
            <p class="mt-4 text-text-secondary">"Page not found"</p>
            <a href="/" class="mt-6 text-primary hover:underline">
                "Go back home"
            </a>
        </div>
    }
}
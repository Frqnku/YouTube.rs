use leptos::prelude::*;
use leptos_meta::{Meta, provide_meta_context};
use leptos_router::StaticSegment;
use leptos_router::components::{Route, Router, Routes};

use crate::components::ui::{Navbar, Sidebar};
use crate::routes::protected::{PageHistory, PageLikedVideos};
use crate::routes::public::PageHome;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Meta name="viewport" content="width=device-width, initial-scale=1.0, viewport-fit=cover" />

        <Router>
            <Navbar />
            <div class="flex min-h-[calc(100dvh-3.5rem)] bg-bg">
                <Sidebar />
                <main class="flex-1">
                    <Routes fallback=|| PageNotFound.into_view()>
                        <Route path=StaticSegment("") view=PageHome />
                        <Route path=StaticSegment("history") view=PageHistory />
                        <Route path=StaticSegment("liked-videos") view=PageLikedVideos />
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
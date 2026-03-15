use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

use crate::api::_errors::AppServerError;
use crate::api::_dtos::video::VideoPlayer;
use crate::api::video::get_video::get_video;
use crate::components::ui::{Loader, NotFound};
use crate::components::videos::{NextVideos, video_player::WatchVideo};

#[component]
fn WatchPageLayout(video: VideoPlayer) -> impl IntoView {
    view! {
        <section class="mx-auto grid w-full max-w-7xl gap-6 xl:grid-cols-[minmax(0,1fr)_360px]">
            <WatchVideo video=video />
            <NextVideos />
        </section>
    }
}

#[component]
pub fn WatchPage() -> impl IntoView {
    let query_map = use_query_map();
    let video_id = move || query_map.with(|qm| qm.get("v").map(|value| value.to_string()));

    let video_resource = Resource::new(
        move || video_id(),
        move |maybe_id| async move {
            match maybe_id {
                Some(id) => get_video(id).await,
                None => Err(AppServerError::new("video_not_found", "This video isn't available")),
            }
        },
    );

    view! {
        <main class="min-h-[calc(100dvh-3.5rem)] bg-bg px-4 py-6 md:px-6">
            <Suspense
                fallback=move || {
                    view! { <Loader /> }
                }
            >
                {move || {
                    video_resource
                        .get()
                        .map(|result| match result {
                            Ok(video) => view! { <WatchPageLayout video=video /> }.into_any(),
                            Err(error) => view! { <NotFound message=error.message /> }.into_any(),
                        })
                }}
            </Suspense>
        </main>
    }
}
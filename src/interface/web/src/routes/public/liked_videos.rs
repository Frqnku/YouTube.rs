use leptos::prelude::*;
use crate::api::video::get_liked_videos;
use crate::components::videos::VideoCard;
use crate::components::videos::video_feed::{ResponsiveVideoCardSkeletons, use_paginated_feed};

const LIKED_PAGE_SIZE: u32 = 6;

#[component]
pub fn LikedVideosPage() -> impl IntoView {
    let (
        videos,
        _next_cursor,
        _has_more,
        initial_error,
        load_more_error,
        load_more,
    ) = use_paginated_feed(
        Signal::derive(|| ()),
        |(), cursor| async move {
            get_liked_videos(Some(LIKED_PAGE_SIZE), cursor)
                .await
                .map_err(|_| ())
        },
    );

    view! {
        <div class="min-h-[calc(100dvh-3.5rem)] bg-bg px-4 py-5 md:px-6">
            <h1 class="text-2xl font-semibold text-text">"Liked videos"</h1>
            <p class="mt-1 text-sm text-text-secondary">"Videos you've liked"</p>

            <section class="mt-6 grid grid-cols-1 gap-6 lg:grid-cols-2 2xl:grid-cols-3" data-section="history-grid">
                <Suspense
                    fallback=move || {
                        view! { <ResponsiveVideoCardSkeletons /> }.into_any()
                    }
                >
                    {move || {
                        if initial_error.get() {
                            return view! {
                                <article class="col-span-full rounded-xl bg-bg-secondary p-4 text-sm text-text-secondary">
                                    "Unable to load your liked videos right now. Please try again later."
                                </article>
                            }
                                .into_any()
                                .into_view();
                        }

                        if videos.get().is_empty() {
                            return view! {
                                <article class="col-span-full rounded-xl bg-bg-secondary p-4 text-sm text-text-secondary">
                                    "No liked videos yet. Start liking videos and they will appear here."
                                </article>
                            }
                                .into_any()
                                .into_view();
                        }

                        view! {
                            <For
                                each=move || videos.get()
                                key=|video| video.id.clone()
                                children=move |video| {
                                    view! { <VideoCard video=video /> }
                                }
                            />
                        }
                            .into_any()
                            .into_view()
                    }}
                </Suspense>
                <Show when=move || load_more.pending().get()>
                    <ResponsiveVideoCardSkeletons />
                </Show>
            </section>


            <Show when=move || load_more_error.get()>
                <div class="pb-5 text-center text-sm text-text-secondary">
                    "Couldn't load more liked videos. Keep scrolling to retry."
                </div>
            </Show>
        </div>
    }
}
use leptos::prelude::*;
use crate::api::video::{get_newest_videos, get_trending_videos};
use crate::components::ui::Loader;
use crate::components::videos::VideoCard;
use crate::components::videos::video_feed::{ResponsiveVideoCardSkeletons, use_paginated_feed};

const HOME_PAGE_SIZE: u32 = 12;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HomeFeed {
    Trending,
    New,
}

#[component]
fn HomeFilters(
    current_feed: RwSignal<HomeFeed>,
) -> impl IntoView {
    view! {
        <div
            class="sticky top-14 z-30 -mx-4 mb-5 bg-bg/95 px-4 py-3 backdrop-blur md:-mx-6 md:px-6"
            data-section="home-filters"
        >
            <div class="flex flex-wrap items-center gap-2">
                <button
                    class="btn-tertiary"
                    class:bg-text=move || current_feed.get() == HomeFeed::Trending
                    class:text-bg=move || current_feed.get() == HomeFeed::Trending
                    on:click=move |_| current_feed.set(HomeFeed::Trending)
                >
                    "Trending"
                </button>
                <button
                    class="btn-tertiary"
                    class:bg-text=move || current_feed.get() == HomeFeed::New
                    class:text-bg=move || current_feed.get() == HomeFeed::New
                    on:click=move |_| current_feed.set(HomeFeed::New)
                >
                    "New"
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn HomePage() -> impl IntoView {
    let current_feed = RwSignal::new(HomeFeed::Trending);
    let (
        videos,
        _next_cursor,
        _has_more,
        initial_error,
        load_more_error,
        load_more,
    ) = use_paginated_feed(
        Signal::derive(move || current_feed.get()),
        |feed, cursor| async move {
            match feed {
                HomeFeed::Trending => get_trending_videos(Some(HOME_PAGE_SIZE), cursor).await.map_err(|_| ()),
                HomeFeed::New => get_newest_videos(Some(HOME_PAGE_SIZE), cursor).await.map_err(|_| ()),
            }
        },
    );

    view! {
        <div class="min-h-[calc(100dvh-3.5rem)] bg-bg px-4 pb-5 md:px-6">
            <HomeFilters current_feed=current_feed />

            <section class="grid grid-cols-1 gap-6 lg:grid-cols-2 2xl:grid-cols-3" data-section="video-grid">
                <Suspense
                    fallback=move || {
                        view! { <ResponsiveVideoCardSkeletons /> }.into_any()
                    }
                >
                    {move || {
                        if initial_error.get() {
                            return view! {
                                <article class="col-span-full rounded-xl bg-bg-secondary p-4 text-sm text-text-secondary">
                                    "Unable to load videos right now. Please try again later."
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
            </section>

            <Show when=move || load_more.pending().get()>
                <ResponsiveVideoCardSkeletons />
                <Loader />
            </Show>

            <Show when=move || load_more_error.get()>
                <div class="pb-5 text-center text-sm text-text-secondary">
                    "Couldn't load more videos. Keep scrolling to retry."
                </div>
            </Show>
        </div>
    }
}

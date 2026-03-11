use leptos::prelude::*;
use crate::api::_dtos::video::VideoCardDto;
use crate::api::video::{get_newest_videos, get_trending_videos};
#[cfg(target_arch = "wasm32")]
use crate::components::_helpers::is_near_bottom_of_page;
use crate::components::ui::Loader;
use crate::components::video::{VideoCard, VideoCardSkeleton};

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
                    class="btn-secondary"
                    class:bg-text=move || current_feed.get() == HomeFeed::Trending
                    class:text-bg=move || current_feed.get() == HomeFeed::Trending
                    on:click=move |_| current_feed.set(HomeFeed::Trending)
                >
                    "Trending"
                </button>
                <button
                    class="btn-secondary"
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
fn ResponsiveVideoCardSkeletons() -> impl IntoView {
    view! {
        <For
            each=move || 0..6
            key=|index| *index
            children=move |index| {
                let visibility_class = match index {
                    0 | 1 => "block",
                    2 | 3 => "hidden lg:block",
                    _ => "hidden 2xl:block",
                };

                view! {
                    <div class=visibility_class>
                        <VideoCardSkeleton />
                    </div>
                }
            }
        />
    }
}


#[component]
pub fn HomePage() -> impl IntoView {
    let current_feed = RwSignal::new(HomeFeed::Trending);
    let videos = RwSignal::new(Vec::<VideoCardDto>::new());
    let next_cursor = RwSignal::new(None::<String>);
    let has_more = RwSignal::new(false);
    let initial_error = RwSignal::new(false);
    let load_more_error = RwSignal::new(false);

    let newest_videos = Resource::new(
        move || current_feed.get(),
        move |feed| async move {
            match feed {
                HomeFeed::Trending => get_trending_videos(Some(HOME_PAGE_SIZE), None).await,
                HomeFeed::New => get_newest_videos(Some(HOME_PAGE_SIZE), None).await,
            }
        },
    );

    let load_more = Action::new(move |(feed, cursor): &(HomeFeed, String)| {
        let feed = *feed;
        let cursor = cursor.clone();
        async move {
            match feed {
                HomeFeed::Trending => get_trending_videos(Some(HOME_PAGE_SIZE), Some(cursor)).await,
                HomeFeed::New => get_newest_videos(Some(HOME_PAGE_SIZE), Some(cursor)).await,
            }
        }
    });

    Effect::new(move |_| {
        let Some(result) = newest_videos.get() else {
            return;
        };

        match result {
            Ok(page) => {
                initial_error.set(false);
                videos.set(page.items);
                next_cursor.set(page.next_cursor);
                has_more.set(page.has_more);
            }
            Err(_) => {
                initial_error.set(true);
                videos.set(Vec::new());
                next_cursor.set(None);
                has_more.set(false);
            }
        }
    });

    Effect::new(move |_| {
        let Some(result) = load_more.value().get() else {
            return;
        };

        match result {
            Ok(page) => {
                load_more_error.set(false);
                videos.update(|items| items.extend(page.items));
                next_cursor.set(page.next_cursor);
                has_more.set(page.has_more);
            }
            Err(_) => {
                load_more_error.set(true);
            }
        }
    });

    #[cfg(target_arch = "wasm32")]
    {
        let window_scroll_listener = window_event_listener(leptos::ev::scroll, move |_| {
            if !is_near_bottom_of_page() {
                return;
            }

            if !has_more.get_untracked() || load_more.pending().get_untracked() {
                return;
            }

            if let Some(cursor) = next_cursor.get_untracked() {
                load_more_error.set(false);
                load_more.dispatch((current_feed.get_untracked(), cursor));
            }
        });

        // Keep the listener alive for the component lifetime.
        StoredValue::new(window_scroll_listener);
    }

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
                                <article class="col-span-full rounded-xl border border-border bg-bg-secondary p-4 text-sm text-text-secondary">
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

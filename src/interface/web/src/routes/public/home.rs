use leptos::prelude::*;
use leptos_router::hooks::use_location;
use wasm_bindgen::prelude::*;
use crate::api::video::{get_newest_videos, get_trending_videos, get_videos_by_tag};
use crate::components::videos::VideoCard;
use crate::components::videos::video_feed::{ResponsiveVideoCardSkeletons, use_paginated_feed};

const HOME_PAGE_SIZE: u32 = 6;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tag {
    Music,
    Challenge,
    Entertainment,
    Gaming,
    Horror,
    Vlog,
    Tech,
    Rust,
    Cybersecurity,
    AI,
    Anime,
    Netflix,
}

impl Tag {
    fn all() -> &'static [Tag] {
        &[
            Tag::Music,
            Tag::Challenge,
            Tag::Entertainment,
            Tag::Gaming,
            Tag::Horror,
            Tag::Vlog,
            Tag::Tech,
            Tag::Rust,
            Tag::Cybersecurity,
            Tag::AI,
            Tag::Anime,
            Tag::Netflix,
        ]
    }

    fn label(self) -> &'static str {
        match self {
            Tag::Music => "Music",
            Tag::Challenge => "Challenge",
            Tag::Entertainment => "Entertainment",
            Tag::Gaming => "Gaming",
            Tag::Horror => "Horror",
            Tag::Vlog => "Vlog",
            Tag::Tech => "Tech",
            Tag::Rust => "Rust",
            Tag::Cybersecurity => "Cybersecurity",
            Tag::AI => "AI",
            Tag::Anime => "Anime",
            Tag::Netflix => "Netflix",
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Tag::Music => "music",
            Tag::Challenge => "challenge",
            Tag::Entertainment => "entertainment",
            Tag::Gaming => "gaming",
            Tag::Horror => "horror",
            Tag::Vlog => "vlog",
            Tag::Tech => "tech",
            Tag::Rust => "rust",
            Tag::Cybersecurity => "cybersecurity",
            Tag::AI => "ai",
            Tag::Anime => "anime",
            Tag::Netflix => "netflix",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HomeFeed {
    Trending,
    New,
    Tag(Tag),
}

#[component]
fn HomeFilters(
    current_feed: RwSignal<HomeFeed>,
) -> impl IntoView {
    let (is_md_or_larger, set_is_md_or_larger) = signal(false);

    Effect::new(move |_| {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(mq)) = window.match_media("(min-width: 768px)") {
                set_is_md_or_larger.set(mq.matches());

                let callback = Closure::wrap(Box::new(move |_: web_sys::Event| {
                    if let Some(window) = web_sys::window() {
                        if let Ok(Some(m)) = window.match_media("(min-width: 768px)") {
                            set_is_md_or_larger.set(m.matches());
                        }
                    }
                }) as Box<dyn Fn(web_sys::Event)>);

                let _ = mq.add_listener_with_opt_callback(Some(callback.as_ref().unchecked_ref()));
                callback.forget();
            }
        }
    });

    view! {
        <div
            class="sticky top-14 z-30 mb-5 bg-bg/95 py-3 backdrop-blur"
            data-section="home-filters"
        >
            <div class="flex flex-wrap items-center gap-3">
                <button
                    class="btn-tertiary font-semibold"
                    class:btn-tertiary-active=move || current_feed.get() == HomeFeed::Trending
                    on:click=move |_| current_feed.set(HomeFeed::Trending)
                >
                    "Trending"
                </button>
                <button
                    class="btn-tertiary font-semibold"
                    class:btn-tertiary-active=move || current_feed.get() == HomeFeed::New
                    on:click=move |_| current_feed.set(HomeFeed::New)
                >
                    "New"
                </button>
                <Show when=move || is_md_or_larger.get()>
                    <For
                        each=|| Tag::all().to_vec()
                        key=|&tag| format!("{:?}", tag)
                        children=move |tag| {
                            view! {
                                <button
                                    class="btn-tertiary font-semibold"
                                    class:btn-tertiary-active=move || current_feed.get() == HomeFeed::Tag(tag)
                                    on:click=move |_| current_feed.set(HomeFeed::Tag(tag))
                                >
                                    {tag.label()}
                                </button>
                            }
                        }
                    />
                </Show>
            </div>
        </div>
    }
}

#[component]
fn HomePageContent() -> impl IntoView {
    let current_feed = RwSignal::new(HomeFeed::Trending);
    let (
        videos,
        _next_cursor,
        _has_more,
        _initial_loaded,
        initial_error,
        load_more_error,
        load_more,
    ) = use_paginated_feed(
        Signal::derive(move || current_feed.get()),
        |feed, cursor| async move {
            match feed {
                HomeFeed::Trending => get_trending_videos(Some(HOME_PAGE_SIZE), cursor).await.map_err(|_| ()),
                HomeFeed::New => get_newest_videos(Some(HOME_PAGE_SIZE), cursor).await.map_err(|_| ()),
                HomeFeed::Tag(tag) => get_videos_by_tag(tag.as_str().to_string(), Some(HOME_PAGE_SIZE), cursor).await.map_err(|_| ()),
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
                <Show when=move || load_more.pending().get()>
                    <ResponsiveVideoCardSkeletons />
                </Show>
            </section>


            <Show when=move || load_more_error.get()>
                <div class="pb-5 text-center text-sm text-text-secondary">
                    "Couldn't load more videos. Keep scrolling to retry."
                </div>
            </Show>
        </div>
    }
}

#[component]
pub fn HomePage() -> impl IntoView {
    let location = use_location();

    view! {
        <Show
            when=move || location.pathname.get() == "/"
            fallback=move || view! { <div class="hidden"></div> }.into_any()
        >
            <HomePageContent />
        </Show>
    }
}

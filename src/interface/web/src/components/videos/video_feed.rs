use std::future::Future;

use leptos::prelude::*;

use crate::{api::_dtos::video::{VideoCardDto, VideoCardPage}, components::videos::VideoCardSkeleton};
#[cfg(target_arch = "wasm32")]
use crate::components::_helpers::is_near_bottom_of_page;

#[component]
pub fn ResponsiveVideoCardSkeletons() -> impl IntoView {
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

pub fn use_paginated_feed<Key, FetchFn, Fut>(
    key: Signal<Key>,
    fetch_page: FetchFn,
) -> (
    RwSignal<Vec<VideoCardDto>>,
    RwSignal<Option<String>>,
    RwSignal<bool>,
    RwSignal<bool>,
    RwSignal<bool>,
    RwSignal<bool>,
    Action<(Key, String), Result<VideoCardPage, ()>>,
)
where
    Key: Clone + PartialEq + Send + Sync + 'static,
    FetchFn: Fn(Key, Option<String>) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Result<VideoCardPage, ()>> + Send + 'static,
{
    let videos = RwSignal::new(Vec::<VideoCardDto>::new());
    let next_cursor = RwSignal::new(None::<String>);
    let has_more = RwSignal::new(false);
    let initial_loaded = RwSignal::new(false);
    let initial_error = RwSignal::new(false);
    let load_more_error = RwSignal::new(false);

    let list_fetch_page = fetch_page.clone();
    let load_more_fetch_page = fetch_page;

    let list_videos = Resource::new(
        move || key.get(),
        move |selected_key| {
            let fetch_page = list_fetch_page.clone();
            async move { fetch_page(selected_key, None).await }
        },
    );

    let load_more = Action::new(move |(selected_key, cursor): &(Key, String)| {
        let fetch_page = load_more_fetch_page.clone();
        let selected_key = selected_key.clone();
        let cursor = cursor.clone();
        async move { fetch_page(selected_key, Some(cursor)).await }
    });

    Effect::new(move |_| {
        let Some(result) = list_videos.get() else {
            return;
        };

        match result {
            Ok(page) => {
                initial_loaded.set(true);
                initial_error.set(false);
                videos.set(page.items);
                next_cursor.set(page.next_cursor);
                has_more.set(page.has_more);
            }
            Err(_) => {
                initial_loaded.set(true);
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
                load_more.dispatch((key.get_untracked(), cursor));
            }
        });

        // Explicitly drop the listener when the scope is disposed to prevent
        // it from firing on unrelated pages after SPA navigation.
        on_cleanup(move || drop(window_scroll_listener));
    }

    (
        videos,
        next_cursor,
        has_more,
        initial_loaded,
        initial_error,
        load_more_error,
        load_more,
    )
}

pub use use_paginated_feed as use_paginated_video_feed;

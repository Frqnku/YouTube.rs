use leptos::prelude::*;
use crate::api::_dtos::video::VideoCardDto;
use crate::api::video::get_newest_videos;

const HOME_PAGE_SIZE: u32 = 12;
#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
const LOAD_MORE_THRESHOLD_PX: f64 = 1.0;

fn format_duration(total_seconds: i32) -> String {
    let safe_seconds = total_seconds.max(0);
    let minutes = safe_seconds / 60;
    let seconds = safe_seconds % 60;
    format!("{minutes}:{seconds:02}")
}

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
fn is_near_bottom_of_page() -> bool {
    let Some(window) = web_sys::window() else {
        return false;
    };

    let Some(document) = window.document() else {
        return false;
    };

    let scroll_y = window.scroll_y().ok().unwrap_or(0.0);
    let viewport_height = window
        .inner_height()
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);

    let page_height = document
        .document_element()
        .map(|el| el.scroll_height() as f64)
        .or_else(|| document.body().map(|body| body.scroll_height() as f64))
        .unwrap_or(0.0);

    let remaining = page_height - (scroll_y + viewport_height);
    remaining <= LOAD_MORE_THRESHOLD_PX
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
fn is_near_bottom_of_page() -> bool {
    false
}

#[component]
pub fn HomePage() -> impl IntoView {
    let videos = RwSignal::new(Vec::<VideoCardDto>::new());
    let next_cursor = RwSignal::new(None::<String>);
    let has_more = RwSignal::new(false);
    let initial_error = RwSignal::new(false);
    let load_more_error = RwSignal::new(false);

    let newest_videos = Resource::new(
        move || (),
        move |_| async move { get_newest_videos(Some(HOME_PAGE_SIZE), None).await },
    );

    let load_more = Action::new(move |cursor: &String| {
        let cursor = cursor.clone();
        async move { get_newest_videos(Some(HOME_PAGE_SIZE), Some(cursor)).await }
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
                load_more.dispatch(cursor);
            }
        });

        // Keep the listener alive for the component lifetime.
        StoredValue::new(window_scroll_listener);
    }

    view! {
        <div class="min-h-[calc(100dvh-3.5rem)] bg-bg px-4 py-5 md:px-6">
            // Home filters section
            <div class="mb-5 flex flex-wrap items-center gap-2" data-section="home-filters">
                <button class="btn-secondary bg-text text-bg">"All"</button>
                <button class="btn-secondary">"Rust"</button>
                <button class="btn-secondary">"Leptos"</button>
                <button class="btn-secondary">"Programming"</button>
                <button class="btn-secondary">"Architecture"</button>
                <button class="btn-secondary">"New"</button>
            </div>

            // Videos grid section
            <section class="grid grid-cols-1 gap-6 sm:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4" data-section="video-grid">
                <Suspense
                    fallback=move || {
                        view! {
                            <For
                                each=move || 0..8
                                key=|index| *index
                                children=move |_| {
                                    view! {
                                        <article class="group animate-pulse">
                                            <div class="relative aspect-video overflow-hidden rounded-xl bg-bg-secondary">
                                                <div class="h-full w-full bg-bg-tertiary" />
                                            </div>
                                            <div class="mt-3 flex gap-3">
                                                <div class="mt-1 h-9 w-9 shrink-0 rounded-full bg-bg-tertiary" />
                                                <div class="min-w-0 w-full space-y-2">
                                                    <div class="h-4 w-11/12 rounded bg-bg-tertiary" />
                                                    <div class="h-3 w-2/3 rounded bg-bg-tertiary" />
                                                </div>
                                            </div>
                                        </article>
                                    }
                                }
                            />
                        }
                            .into_any()
                    }
                >
                    {move || {
                        if initial_error.get() {
                            return view! {
                                <article class="col-span-full rounded-xl border border-border bg-bg-secondary p-4 text-sm text-text-secondary">
                                    "Unable to load newest videos right now."
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
                                    let duration = format_duration(video.duration_seconds);
                                    let meta = format!("{} views", video.view_count);
                                    view! {
                                        <article class="group">
                                            <a href="/watch" class="block">
                                                <div class="relative aspect-video overflow-hidden rounded-xl bg-bg-secondary">
                                                    <img
                                                        src=video.thumbnail_url
                                                        alt=video.title.clone()
                                                        class="h-full w-full object-cover transition group-hover:scale-[1.02]"
                                                    />
                                                    <span class="absolute bottom-2 right-2 rounded bg-black/80 px-1.5 py-0.5 text-xs font-medium text-white">
                                                        {duration}
                                                    </span>
                                                </div>

                                                <div class="mt-3 flex gap-3">
                                                    <img
                                                        src=video.user_picture.clone().unwrap_or_default()
                                                        alt=format!("{}'s profile picture", video.user)
                                                        class="mt-1 h-9 w-9 shrink-0 rounded-full bg-bg-secondary object-cover"
                                                    />
                                                    <div class="min-w-0">
                                                        <h3 class="line-clamp-2 text-sm font-medium text-text">{video.title}</h3>
                                                        <p class="mt-1 text-sm text-text-secondary">{video.user}</p>
                                                        <p class="text-sm text-text-muted">{meta}</p>
                                                    </div>
                                                </div>
                                            </a>
                                        </article>
                                    }
                                }
                            />
                        }
                            .into_any()
                            .into_view()
                    }}
                </Suspense>
            </section>

            // Infinite scroll loading section
            <Show when=move || load_more.pending().get()>
                <div class="py-5 text-center text-sm text-text-muted">
                    "Loading more videos..."
                </div>
            </Show>

            // Infinite scroll terminal states section
            <Show when=move || load_more_error.get()>
                <div class="pb-5 text-center text-sm text-text-secondary">
                    "Couldn't load more videos. Keep scrolling to retry."
                </div>
            </Show>

            <Show when=move || !has_more.get() && !videos.get().is_empty()>
                <div class="pb-5 text-center text-sm text-text-muted">
                    "You've reached the end."
                </div>
            </Show>
        </div>
    }
}

use leptos::prelude::*;
use crate::api::_dtos::video::VideoCardDto;
use crate::api::video::get_newest_videos;
#[cfg(target_arch = "wasm32")]
use crate::components::_helpers::is_near_bottom_of_page;
use crate::components::ui::Loader;
use crate::components::video::{VideoCard, VideoCardSkeleton};

const HOME_PAGE_SIZE: u32 = 12;

#[component]
fn HomeFilters() -> impl IntoView {
    view! {
        <div class="mb-5 flex flex-wrap items-center gap-2" data-section="home-filters">
            <button class="btn-secondary bg-text text-bg">"All"</button>
            <button class="btn-secondary">"Rust"</button>
            <button class="btn-secondary">"Leptos"</button>
            <button class="btn-secondary">"Programming"</button>
            <button class="btn-secondary">"Architecture"</button>
            <button class="btn-secondary">"New"</button>
        </div>
    }
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
            <HomeFilters />

            <section class="grid grid-cols-1 gap-6 lg:grid-cols-2 2xl:grid-cols-3" data-section="video-grid">
                <Suspense
                    fallback=move || {
                        view! {
                            <For
                                each=move || 0..8
                                key=|index| *index
                                children=move |_| {
                                    view! { <VideoCardSkeleton /> }
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
                <Loader />
            </Show>

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

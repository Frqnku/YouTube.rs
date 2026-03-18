use leptos::prelude::*;
use crate::api::video::get_history_videos;
use crate::app::CurrentUserContext;
use crate::components::ui::icons::IconKind;
use crate::components::videos::VideoCard;
use crate::components::videos::video_feed::{ResponsiveVideoCardSkeletons, use_paginated_feed};
use crate::components::ui::RequireAuth;

const HISTORY_PAGE_SIZE: u32 = 6;

#[component]
pub fn HistoryPage() -> impl IntoView {
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
            get_history_videos(Some(HISTORY_PAGE_SIZE), cursor)
                .await
                .map_err(|_| ())
        },
    );

    let current_user_ctx = use_context::<CurrentUserContext>();
    let is_authenticated = Signal::derive(move || {
        current_user_ctx
            .as_ref()
            .and_then(|ctx| ctx.current_user.get())
            .is_some()
    });

    view! {
        <div class="min-h-[calc(100dvh-3.5rem)] bg-bg px-4 py-5 md:px-6">
            <Show when=move || is_authenticated.get()
                fallback=move || view! {
                    <RequireAuth
                        icon_kind=IconKind::History
                        title="Keep track of what you watch".to_string()
                        message="Watch history isn't viewable when signed out.".to_string()
                    />
                }.into_any()
            >
                <h1 class="text-2xl font-semibold text-text">"History"</h1>
                <p class="mt-1 text-sm text-text-secondary">"Recently watched videos"</p>

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
                                        "Unable to load your history right now. Please try again later."
                                    </article>
                                }
                                    .into_any()
                                    .into_view();
                            }

                            if videos.get().is_empty() {
                                return view! {
                                    <article class="col-span-full rounded-xl bg-bg-secondary p-4 text-sm text-text-secondary">
                                        "No history yet. Start watching videos and they will appear here."
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
                        "Couldn't load more history videos. Keep scrolling to retry."
                    </div>
                </Show>
            </Show>
        </div>
    }
}
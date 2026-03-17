use leptos::prelude::*;

use crate::api::video::get_random_videos;
use crate::components::_helpers::{CountFormat, format_count, format_duration, format_relative_time};
use crate::components::ui::DotDivider;
use crate::components::ui::icons::{Icon, IconKind};

#[component]
pub fn NextVideos(current_video_id: String, next_video_url: RwSignal<Option<String>>) -> impl IntoView {
    let current_video_id_for_fetch = std::sync::Arc::new(current_video_id.clone());

    let next_videos = Resource::new(
        || (),
        move |_| {
            let exclude_video_id = current_video_id_for_fetch.as_ref().clone();
            async move { get_random_videos(Some(6), Some(exclude_video_id)).await }
        },
    );

    Effect::new(move |_| {
        match next_videos.get() {
            Some(Ok(page)) => {
                let next_url = page
                    .items
                    .iter()
                    .find(|video| video.id != current_video_id)
                    .map(|video| format!("/watch?v={}", video.id));
                next_video_url.set(next_url);
            }
            Some(Err(_)) => {
                next_video_url.set(None);
            }
            None => {}
        }
    });

    view! {
        <aside class="space-y-3">
            <h2 class="text-sm font-semibold uppercase tracking-wide text-text-secondary">
                "Next videos"
            </h2>
            <Suspense fallback=move || view! { <div class="space-y-2" /> }>
                {move || {
                    next_videos
                        .get()
                        .map(|result| match result {
                            Ok(page) => view! {
                                <div class="space-y-2">
                                    <For
                                        each=move || page.items.clone().into_iter()
                                        key=|video| video.id.clone()
                                        children=move |video| {
                                            let duration = format_duration(video.duration_seconds);
                                            let watch_url = format!("/watch?v={}", video.id);
                                            let uploaded_ago = format_relative_time(&video.uploaded_at);
                                            let view_count = format!(
                                                "{} views",
                                                format_count(video.view_count, CountFormat::Short),
                                            );

                                            view! {
                                                <a class="flex justify-between gap-3" href=watch_url>
                                                    <div class="relative aspect-video w-42 shrink-0 overflow-hidden rounded-lg bg-bg-tertiary">
                                                        <img
                                                            src=video.thumbnail_url
                                                            alt=video.title.clone()
                                                            class="h-full w-full object-cover"
                                                        />
                                                        <span class="absolute bottom-1.5 right-1.5 rounded bg-black/80 px-1.5 py-0.5 text-[11px] font-medium text-white">
                                                            {duration}
                                                        </span>
                                                    </div>
                                                    <div class="min-w-0">
                                                        <p class="line-clamp-2 text-sm font-medium text-text">{video.title}</p>
                                                        <p class="mt-1 text-xs text-text-secondary">{video.user}</p>
                                                        <p class="text-xs text-text-muted">{view_count}<DotDivider />{uploaded_ago}</p>
                                                    </div>
                                                    <div class="flex">
                                                        <Icon kind=IconKind::DotMenu />
                                                    </div>
                                                </a>
                                            }
                                        }
                                    />
                                </div>
                            }
                                .into_any(),
                            Err(_) => view! {
                                <div class="rounded-xl bg-bg-secondary p-3 text-xs text-text-secondary">
                                    "Unable to load next videos."
                                </div>
                            }
                                .into_any(),
                        })
                }}
            </Suspense>
        </aside>
    }
}
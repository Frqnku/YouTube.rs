use leptos::prelude::*;

use crate::{api::_dtos::video::VideoCardDto, components::{_helpers::{CountFormat, format_count, format_duration, format_relative_time}, ui::DotDivider}};

#[component]
pub fn VideoCardSkeleton() -> impl IntoView {
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

#[component]
pub fn VideoCard(video: VideoCardDto) -> impl IntoView {
    let duration = format_duration(video.duration_seconds);
    let uploaded_ago = format_relative_time(&video.uploaded_at);
    let watch_url = format!("/watch?v={}", video.id);
    let view_count = format!("{} views", format_count(video.view_count, CountFormat::Short));

    view! {
        <article class="group">
            <a href=watch_url class="block">
                <div class="relative aspect-video overflow-hidden rounded-xl bg-bg-secondary">
                    <img
                        src=video.thumbnail_url
                        alt=video.title.clone()
                        class="h-full w-full object-cover transition group-hover:hidden"
                    />
                    <img
                        src=video.preview_url
                        alt=video.title.clone()
                        class="h-full w-full object-cover transition group-hover:block group-hover:scale-[1.02]"
                    />
                    <span class="absolute bottom-2 right-2 rounded bg-black/80 px-1.5 py-0.5 text-xs font-medium text-white">
                        {duration}
                    </span>
                    <Show when=move || video.watched_seconds.is_some()>
                        <ProgressBar
                            video_duration_seconds=video.duration_seconds
                            watched_seconds=video.watched_seconds.unwrap()
                        />
                    </Show>
                </div>

                <div class="mt-3 flex gap-3">
                    <img
                        src=video.user_picture.unwrap_or_default()
                        alt=format!("{}'s profile picture", video.user)
                        class="mt-1 h-9 w-9 shrink-0 rounded-full bg-bg-secondary object-cover"
                    />
                    <div class="min-w-0">
                        <h3 class="line-clamp-2 text-sm font-medium text-text">{video.title}</h3>
                        <p class="mt-1 text-sm text-text-secondary">{video.user}</p>
                        <p class="text-sm text-text-muted">{view_count}<DotDivider />{uploaded_ago}</p>
                    </div>
                </div>
            </a>
        </article>
    }
}

#[component]
fn ProgressBar(video_duration_seconds: i32, watched_seconds: i32) -> impl IntoView {
    let watched_seconds = watched_seconds.clamp(0, video_duration_seconds.max(0));
    let watched_progress = if video_duration_seconds > 0 {
        (watched_seconds as f64 / video_duration_seconds as f64) * 100.0
    } else {
        0.0
    };
    let watched_progress_step = if watched_seconds == 0 {
        10.0
    } else {
        ((watched_progress / 10.0).ceil() * 10.0).clamp(0.0, 100.0)
    };

    view! {
        <div class="absolute z-20 inset-x-0 bottom-0 h-1 bg-neutral-500">
            <div
                class="h-full bg-red-600"
                style=format!("width: {:.0}%;", watched_progress_step)
            />
        </div>
    }
}
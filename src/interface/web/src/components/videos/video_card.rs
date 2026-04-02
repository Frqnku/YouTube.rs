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
    let channel_url = format!("/channel?id={}", video.channel_id);
    let view_count = format!("{} views", format_count(video.view_count, CountFormat::Short));
    let is_previewing = RwSignal::new(false);

    let watch_url_for_click = watch_url.clone();
    let thumbnail_url = video.thumbnail_url.clone();
    let preview_url = video.preview_url.clone();
    let title = video.title.clone();
    let user = video.user.clone();
    let user_picture = video.user_picture.clone().unwrap_or_default();

    view! {
        <article class="group">
            <div
                class="block cursor-pointer"
                on:click=move |_| {
                    if let Some(window) = web_sys::window() {
                        let _ = window.location().set_href(&watch_url_for_click);
                    }
                }
            >
                <div
                    class="relative aspect-video overflow-hidden rounded-xl bg-bg-secondary"
                    on:mouseenter=move |_| is_previewing.set(true)
                    on:mouseleave=move |_| is_previewing.set(false)
                >
                    <img
                        src=move || {
                            if is_previewing.get() {
                                preview_url.clone()
                            } else {
                                thumbnail_url.clone()
                            }
                        }
                        alt=title
                        class=move || {
                            if is_previewing.get() {
                                "h-full w-full object-cover transition-transform duration-200 scale-[1.02]"
                            } else {
                                "h-full w-full object-cover transition-transform duration-200 scale-100"
                            }
                        }
                    />
                    <span class="absolute bottom-2 right-2 rounded bg-black/80 px-1.5 py-0.5 text-xs font-medium text-white">
                        {duration}
                    </span>
                    <Show when=move || video.watched_seconds.is_some()>
                        <ProgressBar
                            video_duration_seconds=video.duration_seconds
                            watched_seconds=video.watched_seconds.unwrap_or(0)
                        />
                    </Show>
                </div>

                <div class="mt-3 flex gap-3">
                    <a
                        href=channel_url.clone()
                        class="mt-1 block h-9 w-9 shrink-0"
                        on:click=move |event| event.stop_propagation()
                    >
                        <img
                            src=user_picture
                            alt=format!("{}'s profile picture", user)
                            class="h-9 w-9 rounded-full bg-bg-secondary object-cover"
                        />
                    </a>
                    <div class="min-w-0">
                        <h3 class="line-clamp-2 text-sm font-medium text-text">{video.title}</h3>
                        <a
                            href=channel_url
                            class="mt-1 inline-block text-sm text-text-secondary hover:text-text"
                            on:click=move |event| event.stop_propagation()
                        >
                            {video.user}
                        </a>
                        <p class="text-sm text-text-muted">{view_count}<DotDivider />{uploaded_ago}</p>
                    </div>
                </div>
            </div>
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
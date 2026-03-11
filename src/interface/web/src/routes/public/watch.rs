use leptos::prelude::*;
use leptos_router::hooks::use_query_map;
use chrono::{DateTime, Utc};

use crate::api::_dtos::video::VideoPlayer as VideoPlayerDto;
use crate::api::_errors::AppServerError;
use crate::api::video::get_video::get_video;

fn format_view_count(count: i64) -> String {
    match count {
        0..=999 => count.to_string(),
        1_000..=999_999 => {
            let thousands = count as f64 / 1_000.0;
            if thousands >= 100.0 {
                format!("{thousands:.0}K")
            } else {
                format!("{thousands:.1}K")
            }
        }
        1_000_000..=999_999_999 => {
            let millions = count as f64 / 1_000_000.0;
            if millions >= 100.0 {
                format!("{millions:.0}M")
            } else {
                format!("{millions:.1}M")
            }
        }
        _ => {
            let billions = count as f64 / 1_000_000_000.0;
            format!("{billions:.1}B")
        }
    }
}

fn format_relative_time(uploaded_at: &str) -> String {
    let Ok(parsed) = DateTime::parse_from_rfc3339(uploaded_at) else {
        return "just now".to_string();
    };

    let now = Utc::now();
    let uploaded_at_utc = parsed.with_timezone(&Utc);
    let delta = now.signed_duration_since(uploaded_at_utc);

    if delta.num_minutes() < 1 {
        return "just now".to_string();
    }

    if delta.num_minutes() < 60 {
        let minutes = delta.num_minutes();
        return if minutes == 1 {
            "1 minute ago".to_string()
        } else {
            format!("{minutes} minutes ago")
        };
    }

    if delta.num_hours() < 24 {
        let hours = delta.num_hours();
        return if hours == 1 {
            "1 hour ago".to_string()
        } else {
            format!("{hours} hours ago")
        };
    }

    if delta.num_days() < 7 {
        let days = delta.num_days();
        return if days == 1 {
            "1 day ago".to_string()
        } else {
            format!("{days} days ago")
        };
    }

    if delta.num_days() < 30 {
        let weeks = delta.num_days() / 7;
        return if weeks == 1 {
            "1 week ago".to_string()
        } else {
            format!("{weeks} weeks ago")
        };
    }

    if delta.num_days() < 365 {
        let months = delta.num_days() / 30;
        return if months == 1 {
            "1 month ago".to_string()
        } else {
            format!("{months} months ago")
        };
    }

    let years = delta.num_days() / 365;
    if years == 1 {
        "1 year ago".to_string()
    } else {
        format!("{years} years ago")
    }
}

#[component]
fn VideoPlayer(video: VideoPlayerDto) -> impl IntoView {
    let view_count = format!("{} views", format_view_count(video.view_count));
    let uploaded_ago = format_relative_time(&video.uploaded_at);
    let like_count = format!("J'aime {}", format_view_count(video.like_count));
    let dislike_count = format!("Je n'aime pas {}", format_view_count(video.dislike_count));

    view! {
        <section class="mx-auto grid w-full max-w-7xl gap-6 xl:grid-cols-[minmax(0,1fr)_360px]">
            <div class="space-y-4">
                <div class="overflow-hidden rounded-xl border border-border bg-bg-secondary">
                    <div class="aspect-video w-full bg-black">
                        <video class="h-full w-full" controls preload="metadata" playsinline>
                            <source src=video.video_url.clone() type="video/mp4" />
                        </video>
                    </div>
                </div>

                <div class="space-y-4">
                    <h1 class="text-xl font-semibold tracking-tight text-text md:text-2xl">
                        {video.title.clone()}
                    </h1>

                    <div class="flex flex-col gap-3 lg:flex-row lg:items-center lg:justify-between">
                        <div class="flex items-center gap-3">
                            <img
                                src=video.user_picture.clone().unwrap_or_default()
                                alt=format!("{}'s profile picture", video.user)
                                class="h-10 w-10 rounded-full bg-bg-tertiary object-cover"
                            />
                            <div>
                                <p class="font-medium text-text">{video.user.clone()}</p>
                                <p class="text-sm text-text-secondary">"Chaîne"</p>
                            </div>
                        </div>

                        <div class="flex flex-wrap gap-2">
                            <button class="btn-primary">"S'abonner"</button>
                            <button class="btn-secondary">{like_count}</button>
                            <button class="btn-secondary">{dislike_count}</button>
                            <button class="btn-secondary">"Partager"</button>
                        </div>
                    </div>

                    <div class="rounded-xl border border-border bg-bg-secondary p-4 text-sm leading-relaxed text-text-secondary">
                        <p class="font-medium text-text">{format!("{} - {}", view_count, uploaded_ago)}</p>
                        <p class="mt-2 whitespace-pre-line">{video.description}</p>
                    </div>
                </div>
            </div>

            <aside class="space-y-3">
                <h2 class="text-sm font-semibold uppercase tracking-wide text-text-secondary">
                    "A suivre"
                </h2>
                <div class="space-y-2">
                    <a class="flex gap-3 rounded-xl border border-border bg-bg-secondary p-2 transition hover:bg-bg-tertiary" href="#">
                        <div class="aspect-video w-40 shrink-0 rounded-lg bg-bg-tertiary" />
                        <div class="min-w-0">
                            <p class="line-clamp-2 text-sm font-medium text-text">"Build a YouTube Clone with Leptos"</p>
                            <p class="mt-1 text-xs text-text-secondary">"Rust Coding"</p>
                            <p class="text-xs text-text-muted">"24K views - 6 hours ago"</p>
                        </div>
                    </a>
                    <a class="flex gap-3 rounded-xl border border-border bg-bg-secondary p-2 transition hover:bg-bg-tertiary" href="#">
                        <div class="aspect-video w-40 shrink-0 rounded-lg bg-bg-tertiary" />
                        <div class="min-w-0">
                            <p class="line-clamp-2 text-sm font-medium text-text">"Leptos Router Deep Dive"</p>
                            <p class="mt-1 text-xs text-text-secondary">"WebAssembly Lab"</p>
                            <p class="text-xs text-text-muted">"17K views - 1 day ago"</p>
                        </div>
                    </a>
                </div>
            </aside>
        </section>
    }
}

#[component]
pub fn WatchPage() -> impl IntoView {
    let query_map = use_query_map();
    let video_id = move || query_map.with(|qm| qm.get("v").map(|value| value.to_string()));

    let video_resource = Resource::new(
        move || video_id(),
        move |maybe_id| async move {
            match maybe_id {
                Some(id) => get_video(id).await,
                None => Err(AppServerError::new("video_not_found", "This video isn't available")),
            }
        },
    );

    view! {
        <main class="min-h-[calc(100dvh-3.5rem)] bg-bg px-4 py-6 md:px-6">
            <Suspense
                fallback=move || {
                    view! {
                        <div class="flex min-h-[calc(100dvh-8rem)] items-center justify-center text-sm text-text-secondary">
                            "Loading video..."
                        </div>
                    }
                }
            >
                {move || {
                    video_resource
                        .get()
                        .map(|result| match result {
                            Ok(video) => view! { <VideoPlayer video=video /> }.into_any(),
                            Err(error) => view! {
                                <div class="flex min-h-[calc(100dvh-8rem)] items-center justify-center">
                                    <div class="max-w-md rounded-xl border border-border bg-bg-secondary p-6 text-center">
                                        <p class="mt-2 text-sm text-text-secondary">{error.message}</p>
                                    </div>
                                </div>
                            }
                                .into_any(),
                        })
                }}
            </Suspense>
        </main>
    }
}
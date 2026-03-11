use leptos::prelude::*;

use crate::{api::_dtos::video::VideoPlayer, components::_helpers::{format_relative_time, format_count}};

#[component]
pub fn WatchVideoPlayer(video: VideoPlayer) -> impl IntoView {
    let view_count = format!("{} views", format_count(video.view_count));
    let uploaded_ago = format_relative_time(&video.uploaded_at);
    let like_count = format!("Like {}", format_count(video.like_count));
    let dislike_count = format!("Dislike {}", format_count(video.dislike_count));

    view! {
        <div class="space-y-4">
            <div class="overflow-hidden rounded-xl bg-bg-secondary">
                <div class="aspect-video w-full bg-black">
                    <video class="h-full w-full" controls preload="metadata" playsinline>
                        <source src=video.video_url type="video/mp4" />
                    </video>
                </div>
            </div>

            <div class="space-y-4">
                <h1 class="text-xl font-semibold tracking-tight text-text md:text-2xl">
                    {video.title}
                </h1>

                <div class="flex flex-col gap-3 lg:flex-row lg:items-center lg:justify-between">
                    <div class="flex items-center gap-3">
                        <img
                            src=video.user_picture.unwrap_or_default()
                            alt=format!("{}'s profile picture", video.user)
                            class="h-10 w-10 rounded-full bg-bg-tertiary object-cover"
                        />
                        <div>
                            <p class="font-medium text-text">{video.user}</p>
                            <p class="text-sm text-text-secondary">"Channel"</p>
                        </div>
                    </div>

                    <div class="flex flex-wrap gap-2">
                        <button class="btn-primary">"Subscribe"</button>
                        <button class="btn-secondary">{like_count}</button>
                        <button class="btn-secondary">{dislike_count}</button>
                        <button class="btn-secondary">"Share"</button>
                    </div>
                </div>

                <div class="rounded-xl bg-bg-tertiary p-4 text-sm leading-relaxed text-text-secondary">
                    <p class="font-medium text-text">{format!("{} - {}", view_count, uploaded_ago)}</p>
                    <p class="mt-2 whitespace-pre-line">{video.description}</p>
                </div>
            </div>
        </div>
    }
}
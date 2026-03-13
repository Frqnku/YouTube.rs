use leptos::prelude::*;

use crate::{
    api::{
        _dtos::video::VideoPlayer,
        video::post_video_view,
    },
    app::CurrentUserContext,
    components::{
        _helpers::{CountFormat, format_count, format_relative_time},
        videos::video_player::ReactionButtons,
    },
};

#[component]
fn SigninPromptModal(
    open: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <Show when=move || open.get()>
            <div
                class="fixed inset-0 z-50 flex items-end justify-center bg-black/45 p-4 md:items-center"
                on:click=move |_| open.set(false)
            >
                <div
                    class="w-full max-w-sm rounded-xl bg-bg-secondary p-4 text-text shadow-xl"
                    on:click=move |event| event.stop_propagation()
                >
                    <p class="text-sm text-text-secondary">"Sign in to like or dislike this video."</p>
                    <div class="mt-4 flex justify-end gap-2">
                        <button
                            type="button"
                            class="btn-secondary"
                            on:click=move |_| open.set(false)
                        >
                            "Later"
                        </button>
                        <a href="/signin" class="btn-primary">"Sign in"</a>
                    </div>
                </div>
            </div>
        </Show>
    }
}

#[component]
pub fn WatchVideoPlayer(video: VideoPlayer) -> impl IntoView {
    let video_for_reactions = video.clone();
    let view_count = format!("{} views", format_count(video.view_count, CountFormat::Long));
    let uploaded_ago = format_relative_time(&video.uploaded_at);

    let current_user_ctx = use_context::<CurrentUserContext>();
    let is_authenticated = Signal::derive(move || {
        current_user_ctx
            .as_ref()
            .and_then(|ctx| ctx.current_user.get())
            .is_some()
    });

    let show_signin_prompt = RwSignal::new(false);

    let view_action = Action::new(|video_id: &String| {
        let video_id = video_id.clone();
        async move { post_video_view(video_id).await }
    });

    Effect::new(move |_| {
        view_action.dispatch(video.id.clone());
    });

    view! {
        <>
        <div class="space-y-4">
            <div class="overflow-hidden rounded-xl bg-bg-secondary">
                <div class="aspect-video w-full bg-black">
                    <video class="h-full w-full" controls preload="metadata" playsinline autoplay>
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
                        <ReactionButtons video=video_for_reactions is_authenticated=is_authenticated show_signin_prompt=show_signin_prompt/>
                        <button class="btn-secondary">"Share"</button>
                    </div>
                </div>

                <div class="rounded-xl bg-bg-tertiary p-4 text-sm leading-relaxed text-text-secondary">
                    <p class="font-medium text-text">{format!("{} - {}", view_count, uploaded_ago)}</p>
                    <p class="mt-2 whitespace-pre-line">{video.description}</p>
                </div>
            </div>
        </div>
        <SigninPromptModal open=show_signin_prompt />
        </>
    }
}
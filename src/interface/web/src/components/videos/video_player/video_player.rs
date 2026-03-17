use leptos::prelude::*;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen::JsCast;
use web_sys::HtmlVideoElement;

use crate::{
    api::{
        _dtos::video::VideoPlayer,
        subscription::get_subscriber_count,
        video::{post_video_view, update_watched_seconds},
    },
    app::CurrentUserContext,
    components::{
        _helpers::{CountFormat, format_count, format_relative_time},
        comments::CommentFeed,
        videos::video_player::{Channel, ReactionButtons, SubscribeButton},
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
pub fn WatchVideo(video: VideoPlayer) -> impl IntoView {
    let video_for_reactions = video.clone();
    let video_id_for_view_action = video.id.clone();
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

    // Subscriber count - initially 0, loaded async
    let subscriber_count = RwSignal::new(0usize);
    let channel_id_for_count = video.channel_id.clone();
    
    // Load initial subscriber count
    let _count_resource = Resource::new(
        move || channel_id_for_count.clone(),
        move |channel_id| async move { get_subscriber_count(channel_id).await },
    );

    Effect::new(move |_| {
        if let Some(Ok(count)) = _count_resource.get() {
            subscriber_count.set(count);
        }
    });

    let view_action = Action::new(|video_id: &String| {
        let video_id = video_id.clone();
        async move { post_video_view(video_id).await }
    });

    Effect::new(move |_| {
        view_action.dispatch(video_id_for_view_action.clone());
    });

    view! {
        <>
        <div class="space-y-4">
            <div class="overflow-hidden rounded-xl bg-bg-secondary">
                <div class="aspect-video w-full bg-black">
                    <VideoPlayer
                        video_url=video.video_url.clone()
                        is_authenticated=is_authenticated
                        video_id=video.id.clone()
                        initial_watched_seconds=video.watched_seconds.unwrap_or_default().max(0) as u32
                    />
                </div>
            </div>

            <div class="space-y-4">
                <h1 class="text-xl font-semibold tracking-tight text-text md:text-2xl">
                    {video.title}
                </h1>

                <div class="flex flex-col gap-3 lg:flex-row lg:items-center lg:justify-between">
                    <div class="flex items-center gap-3">
                        <img
                            src=video.user_picture.clone().unwrap_or_default()
                            alt=format!("{}'s profile picture", video.user)
                            class="h-10 w-10 rounded-full bg-bg-tertiary object-cover"
                        />
                        <Channel
                            channel_name=video.user.clone()
                            subscriber_count=subscriber_count
                        />
                    </div>

                    <div class="flex flex-wrap gap-2">
                        <SubscribeButton
                            channel_id=video.channel_id.clone()
                            is_authenticated=is_authenticated
                            show_signin_prompt=show_signin_prompt
                            subscriber_count=subscriber_count
                        />
                        <ReactionButtons video=video_for_reactions is_authenticated=is_authenticated show_signin_prompt=show_signin_prompt/>
                        <button class="btn-secondary">"Share"</button>
                    </div>
                </div>

                <div class="rounded-xl bg-bg-tertiary p-4 text-sm leading-relaxed text-text-secondary">
                    <p class="font-medium text-text">{format!("{} - {}", view_count, uploaded_ago)}</p>
                    <p class="mt-2 whitespace-pre-line">{video.description}</p>
                </div>

                <CommentFeed video_id=video.id.clone() />
            </div>
        </div>
        <SigninPromptModal open=show_signin_prompt />
        </>
    }
}

#[component]
fn VideoPlayer(video_url: String, is_authenticated: Signal<bool>, video_id: String, initial_watched_seconds: u32) -> impl IntoView {
    let watched_seconds = RwSignal::new(initial_watched_seconds);
    let is_playing = RwSignal::new(false);
    let video_id_for_interval = video_id.clone();

    let update_watched_seconds_action = Action::new(|payload: &(String, u32)| {
        let (video_id, watched_seconds) = payload.clone();
        async move { update_watched_seconds(video_id, watched_seconds).await }
    });

    let make_immediate_update_handler = move |action: Action<(String, u32), _>, video_id: String| {
        let watched_seconds = watched_seconds;
        let is_authenticated = is_authenticated;

        move |event: web_sys::Event| {
            if let Some(target) = event.target() {
                if let Ok(video) = target.dyn_into::<HtmlVideoElement>() {
                    let seconds = video.current_time().floor() as u32;
                    watched_seconds.set(seconds);

                    if is_authenticated.get_untracked() {
                        action.dispatch((video_id.clone(), seconds));
                    }
                }
            }
        }
    };

    let on_play_update = make_immediate_update_handler(
        update_watched_seconds_action.clone(),
        video_id.clone(),
    );
    let on_click_update = make_immediate_update_handler(
        update_watched_seconds_action.clone(),
        video_id.clone(),
    );
    let on_seeked_update = make_immediate_update_handler(
        update_watched_seconds_action.clone(),
        video_id.clone(),
    );

    let make_persist_handler = move |action: Action<(String, u32), _>, video_id: String| {
        let watched_seconds = watched_seconds;
        let is_authenticated = is_authenticated;

        move || {
            if !is_authenticated.get_untracked() {
                return;
            }

            action.dispatch((video_id.clone(), watched_seconds.get_untracked()));
        }
    };

    let on_pause_persist = make_persist_handler(
        update_watched_seconds_action.clone(),
        video_id.clone(),
    );
    let on_ended_persist = make_persist_handler(
        update_watched_seconds_action.clone(),
        video_id.clone(),
    );
    let on_cleanup_persist = make_persist_handler(
        update_watched_seconds_action.clone(),
        video_id.clone(),
    );

    Effect::new(move |_| {
        if !is_playing.get() {
            return;
        }

        let is_playing = is_playing;
        let watched_seconds = watched_seconds;
        let is_authenticated = is_authenticated;
        let update_watched_seconds_action = update_watched_seconds_action.clone();
        let interval_video_id = video_id_for_interval.clone();

        leptos::task::spawn_local(async move {
            while is_playing.get_untracked() {
                TimeoutFuture::new(5000).await;

                if !is_playing.get_untracked() {
                    break;
                }

                if !is_authenticated.get_untracked() {
                    continue;
                }

                update_watched_seconds_action
                    .dispatch((interval_video_id.clone(), watched_seconds.get_untracked()));
            }
        });
    });

    on_cleanup(move || {
        is_playing.set(false);
        on_cleanup_persist();
    });

    view! {
        <video
            class="h-full w-full"
            controls
            preload="metadata"
            playsinline
            autoplay
            on:play=move |event| {
                is_playing.set(true);
                on_play_update(event.unchecked_into::<web_sys::Event>());
            }
            on:pause=move |_| {
                is_playing.set(false);
                on_pause_persist();
            }
            on:ended=move |_| {
                is_playing.set(false);
                on_ended_persist();
            }
            on:click=move |event| {
                on_click_update(event.unchecked_into::<web_sys::Event>());
            }
            on:seeked=move |event| {
                on_seeked_update(event.unchecked_into::<web_sys::Event>());
            }
            on:timeupdate=move |event| {
                if let Some(target) = event.target() {
                    if let Ok(video) = target.dyn_into::<HtmlVideoElement>() {
                        watched_seconds.set(video.current_time().floor() as u32);
                    }
                }
            }
            on:loadedmetadata=move |event| {
                if initial_watched_seconds == 0 {
                    return;
                }

                if let Some(target) = event.target() {
                    if let Ok(video) = target.dyn_into::<HtmlVideoElement>() {
                        let max_seek_time = video.duration().floor().max(0.0) as u32;
                        let resume_at = if initial_watched_seconds >= max_seek_time {
                            0
                        } else {
                            initial_watched_seconds
                        };
                        video.set_current_time(resume_at as f64);
                        watched_seconds.set(resume_at);
                    }
                }
            }
        >
            <source src=video_url type="video/mp4" />
        </video>
    }
}
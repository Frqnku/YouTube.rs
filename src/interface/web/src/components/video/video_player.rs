use leptos::prelude::*;

use crate::{
    api::{
        _dtos::video::VideoPlayer,
        video::{
            delete_video_dislike,
            delete_video_like,
            get_video_reaction,
            post_video_dislike,
            post_video_like,
            post_video_view,
        },
    },
    app::CurrentUserContext,
    components::{
        _helpers::{CountFormat, format_count, format_relative_time},
        ui::icons::{Icon, IconKind},
    },
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ReactionState {
    None,
    Liked,
    Disliked,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ReactionTransition {
    from: ReactionState,
    to: ReactionState,
}

fn next_like_transition(from: ReactionState) -> ReactionTransition {
    let to = if from == ReactionState::Liked {
        ReactionState::None
    } else {
        ReactionState::Liked
    };

    ReactionTransition { from, to }
}

fn next_dislike_transition(from: ReactionState) -> ReactionTransition {
    let to = if from == ReactionState::Disliked {
        ReactionState::None
    } else {
        ReactionState::Disliked
    };

    ReactionTransition { from, to }
}

fn update_reaction_counts(
    like_count: RwSignal<i64>,
    dislike_count: RwSignal<i64>,
    transition: ReactionTransition,
) {
    match (transition.from, transition.to) {
        (ReactionState::None, ReactionState::Liked) => {
            like_count.update(|count| *count += 1);
        }
        (ReactionState::None, ReactionState::Disliked) => {
            dislike_count.update(|count| *count += 1);
        }
        (ReactionState::Liked, ReactionState::None) => {
            like_count.update(|count| *count = (*count - 1).max(0));
        }
        (ReactionState::Disliked, ReactionState::None) => {
            dislike_count.update(|count| *count = (*count - 1).max(0));
        }
        (ReactionState::Liked, ReactionState::Disliked) => {
            like_count.update(|count| *count = (*count - 1).max(0));
            dislike_count.update(|count| *count += 1);
        }
        (ReactionState::Disliked, ReactionState::Liked) => {
            dislike_count.update(|count| *count = (*count - 1).max(0));
            like_count.update(|count| *count += 1);
        }
        _ => {}
    }
}

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
fn LikeButton(
    count_label: Signal<String>,
    selected: Signal<bool>,
    disabled: Signal<bool>,
    on_click: Callback<()>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            class="flex items-center gap-1.5 px-4 py-2 text-sm text-text transition hover:bg-border"
            disabled=move || disabled.get()
            on:click=move |_| on_click.run(())
        >
            {move || {
                if selected.get() {
                    view! { <Icon kind=IconKind::ThumbsUpSelected /> }.into_any()
                } else {
                    view! { <Icon kind=IconKind::ThumbsUp /> }.into_any()
                }
            }}
            <span>{move || count_label.get()}</span>
        </button>
    }
}

#[component]
fn DislikeButton(
    count_label: Signal<String>,
    selected: Signal<bool>,
    disabled: Signal<bool>,
    on_click: Callback<()>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            class="flex items-center gap-1.5 px-4 py-2 text-sm text-text transition hover:bg-border"
            disabled=move || disabled.get()
            on:click=move |_| on_click.run(())
        >
            {move || {
                if selected.get() {
                    view! { <Icon kind=IconKind::ThumbsDownSelected /> }.into_any()
                } else {
                    view! { <Icon kind=IconKind::ThumbsDown /> }.into_any()
                }
            }}
            <span>{move || count_label.get()}</span>
        </button>
    }
}

#[component]
pub fn WatchVideoPlayer(video: VideoPlayer) -> impl IntoView {
    let view_count = format!("{} views", format_count(video.view_count, CountFormat::Long));
    let uploaded_ago = format_relative_time(&video.uploaded_at);
    let video_id_for_status = video.id.clone();
    let video_id_for_like = video.id.clone();
    let video_id_for_dislike = video.id.clone();
    let video_id_for_view = video.id.clone();

    let current_user_ctx = use_context::<CurrentUserContext>();
    let is_authenticated = Signal::derive(move || {
        current_user_ctx
            .as_ref()
            .and_then(|ctx| ctx.current_user.get())
            .is_some()
    });

    let show_signin_prompt = RwSignal::new(false);
    let reaction_state = RwSignal::new(ReactionState::None);
    let like_count = RwSignal::new(video.like_count);
    let dislike_count = RwSignal::new(video.dislike_count);

    let is_liked = Signal::derive(move || reaction_state.get() == ReactionState::Liked);
    let is_disliked = Signal::derive(move || reaction_state.get() == ReactionState::Disliked);

    let like_count_label = Signal::derive(move || format_count(like_count.get(), CountFormat::Short));
    let dislike_count_label = Signal::derive(move || format_count(dislike_count.get(), CountFormat::Short));

    let reaction_status_resource = Resource::new(
        move || (is_authenticated.get(), video_id_for_status.clone()),
        move |(authed, video_id)| async move {
            if !authed {
                return Ok((false, false));
            }

            get_video_reaction(video_id).await
        },
    );

    Effect::new(move |_| {
        let Some(result) = reaction_status_resource.get() else {
            return;
        };

        match result {
            Ok((true, false)) => reaction_state.set(ReactionState::Liked),
            Ok((false, true)) => reaction_state.set(ReactionState::Disliked),
            Ok(_) => reaction_state.set(ReactionState::None),
            Err(_) => {}
        }
    });

    let like_action = Action::new(move |transition: &ReactionTransition| {
        let transition = *transition;
        let video_id = video_id_for_like.clone();
        async move {
            if transition.to == ReactionState::Liked {
                post_video_like(video_id).await
            } else {
                delete_video_like(video_id).await
            }
        }
    });

    let dislike_action = Action::new(move |transition: &ReactionTransition| {
        let transition = *transition;
        let video_id = video_id_for_dislike.clone();
        async move {
            if transition.to == ReactionState::Disliked {
                post_video_dislike(video_id).await
            } else {
                delete_video_dislike(video_id).await
            }
        }
    });

    let reaction_pending = Signal::derive(move || {
        like_action.pending().get() || dislike_action.pending().get()
    });

    let view_action = Action::new(|video_id: &String| {
        let video_id = video_id.clone();
        async move { post_video_view(video_id).await }
    });

    Effect::new(move |_| {
        view_action.dispatch(video_id_for_view.clone());
    });

    let on_like_click = {
        let is_authenticated = is_authenticated;
        Callback::new(move |_| {
            if !is_authenticated.get_untracked() {
                show_signin_prompt.set(true);
                return;
            }

            if reaction_pending.get_untracked() {
                return;
            }

            let transition = next_like_transition(reaction_state.get_untracked());
            reaction_state.set(transition.to);
            update_reaction_counts(like_count, dislike_count, transition);
            like_action.dispatch(transition);
        })
    };

    let on_dislike_click = {
        let is_authenticated = is_authenticated;
        Callback::new(move |_| {
            if !is_authenticated.get_untracked() {
                show_signin_prompt.set(true);
                return;
            }

            if reaction_pending.get_untracked() {
                return;
            }

            let transition = next_dislike_transition(reaction_state.get_untracked());
            reaction_state.set(transition.to);
            update_reaction_counts(like_count, dislike_count, transition);
            dislike_action.dispatch(transition);
        })
    };

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
                        <div class="inline-flex items-center overflow-hidden rounded-full bg-bg-tertiary">
                            <LikeButton
                                count_label=like_count_label
                                selected=is_liked
                                disabled=reaction_pending
                                on_click=on_like_click
                            />
                            <span class="h-5 w-px bg-border bg-bg-tertiary" aria-hidden="true"></span>
                            <DislikeButton
                                count_label=dislike_count_label
                                selected=is_disliked
                                disabled=reaction_pending
                                on_click=on_dislike_click
                            />
                        </div>
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
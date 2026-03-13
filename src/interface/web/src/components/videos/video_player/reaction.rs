use leptos::prelude::*;

use crate::{api::{_dtos::video::VideoPlayer, video::{delete_video_dislike, delete_video_like, get_video_reaction, post_video_dislike, post_video_like}}, components::{_helpers::{CountFormat, format_count}, ui::icons::{Icon, IconKind}}};

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
fn LikeButton(
    video_id: String,
    is_authenticated: Signal<bool>,
    show_signin_prompt: RwSignal<bool>,
    reaction_state: RwSignal<ReactionState>,
    like_count: RwSignal<i64>,
    dislike_count: RwSignal<i64>,
) -> impl IntoView {
    let like_action = Action::new(move |transition: &ReactionTransition| {
        let transition = *transition;
        let video_id = video_id.clone();
        async move {
            if transition.to == ReactionState::Liked {
                post_video_like(video_id).await
            } else {
                delete_video_like(video_id).await
            }
        }
    });

    let like_count_label =
        Signal::derive(move || format_count(like_count.get(), CountFormat::Short));
    let is_liked = Signal::derive(move || reaction_state.get() == ReactionState::Liked);
    let is_pending = like_action.pending();

    let on_click = {
        let is_authenticated = is_authenticated;
        Callback::new(move |_| {
            if !is_authenticated.get_untracked() {
                show_signin_prompt.set(true);
                return;
            }

            if is_pending.get_untracked() {
                return;
            }

            let transition = next_like_transition(reaction_state.get_untracked());
            reaction_state.set(transition.to);
            update_reaction_counts(like_count, dislike_count, transition);
            like_action.dispatch(transition);
        })
    };

    view! {
        <button
            type="button"
            class="flex items-center gap-1.5 px-4 py-2 text-sm text-text transition hover:bg-border"
            disabled=move || is_pending.get()
            on:click=move |_| on_click.run(())
        >
            {move || {
                if is_liked.get() {
                    view! { <Icon kind=IconKind::ThumbsUpSelected /> }.into_any()
                } else {
                    view! { <Icon kind=IconKind::ThumbsUp /> }.into_any()
                }
            }}
            <span>{move || like_count_label.get()}</span>
        </button>
    }
}

#[component]
fn DislikeButton(
    video_id: String,
    is_authenticated: Signal<bool>,
    show_signin_prompt: RwSignal<bool>,
    reaction_state: RwSignal<ReactionState>,
    like_count: RwSignal<i64>,
    dislike_count: RwSignal<i64>,
) -> impl IntoView {
    let dislike_action = Action::new(move |transition: &ReactionTransition| {
        let transition = *transition;
        let video_id = video_id.clone();
        async move {
            if transition.to == ReactionState::Disliked {
                post_video_dislike(video_id).await
            } else {
                delete_video_dislike(video_id).await
            }
        }
    });

    let dislike_count_label =
        Signal::derive(move || format_count(dislike_count.get(), CountFormat::Short));
    let is_disliked = Signal::derive(move || reaction_state.get() == ReactionState::Disliked);
    let is_pending = dislike_action.pending();

    let on_click = {
        let is_authenticated = is_authenticated;
        Callback::new(move |_| {
            if !is_authenticated.get_untracked() {
                show_signin_prompt.set(true);
                return;
            }

            if is_pending.get_untracked() {
                return;
            }

            let transition = next_dislike_transition(reaction_state.get_untracked());
            reaction_state.set(transition.to);
            update_reaction_counts(like_count, dislike_count, transition);
            dislike_action.dispatch(transition);
        })
    };

    view! {
        <button
            type="button"
            class="flex items-center gap-1.5 px-4 py-2 text-sm text-text transition hover:bg-border"
            disabled=move || is_pending.get()
            on:click=move |_| on_click.run(())
        >
            {move || {
                if is_disliked.get() {
                    view! { <Icon kind=IconKind::ThumbsDownSelected /> }.into_any()
                } else {
                    view! { <Icon kind=IconKind::ThumbsDown /> }.into_any()
                }
            }}
            <span>{move || dislike_count_label.get()}</span>
        </button>
    }
}

#[component]
pub fn ReactionButtons(
    video: VideoPlayer,
    is_authenticated: Signal<bool>,
    show_signin_prompt: RwSignal<bool>,
) -> impl IntoView {
    let video_id = video.id.clone();
    let video_id_for_like = video_id.clone();
    let video_id_for_dislike = video_id.clone();

    let reaction_state = RwSignal::new(ReactionState::None);
    let like_count = RwSignal::new(video.like_count);
    let dislike_count = RwSignal::new(video.dislike_count);

    let reaction_status_resource = Resource::new(
        move || (is_authenticated.get(), video_id.clone()),
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

    view! {
        <div class="inline-flex items-center overflow-hidden rounded-full bg-bg-tertiary">
            <LikeButton
                video_id=video_id_for_like
                is_authenticated=is_authenticated
                show_signin_prompt=show_signin_prompt
                reaction_state=reaction_state
                like_count=like_count
                dislike_count=dislike_count
            />
            <span class="h-5 w-px bg-border bg-bg-tertiary" aria-hidden="true"></span>
            <DislikeButton
                video_id=video_id_for_dislike
                is_authenticated=is_authenticated
                show_signin_prompt=show_signin_prompt
                reaction_state=reaction_state
                like_count=like_count
                dislike_count=dislike_count
            />
        </div>
    }
}
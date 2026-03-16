use leptos::prelude::*;

use crate::{
	api::comment::{delete_comment_like, get_comment_like, post_comment_like},
	components::{
		_helpers::{CountFormat, format_count},
		ui::icons::{Icon, IconKind},
	},
};

#[component]
pub fn CommentLikeButton(
	comment_id: String,
	initial_like_count: i64,
	initial_liked_by_viewer: Option<bool>,
	is_authenticated: Signal<bool>,
	show_signin_prompt: RwSignal<bool>,
) -> impl IntoView {
	let like_count = RwSignal::new(initial_like_count.max(0));
	let is_liked = RwSignal::new(initial_liked_by_viewer.unwrap_or(false));
	let comment_id_for_status = comment_id.clone();
	let comment_id_for_click = comment_id.clone();

	let fetch_like_status = Action::new(move |comment_id: &String| {
		let comment_id = comment_id.clone();
		async move { get_comment_like(comment_id).await }
	});

	Effect::new({
		let comment_id_for_status = comment_id_for_status.clone();
		move |_| {
			if is_authenticated.get() {
				fetch_like_status.dispatch(comment_id_for_status.clone());
			}
		}
	});

	Effect::new(move |_| {
		let Some(Ok(liked)) = fetch_like_status.value().get() else {
			return;
		};
		is_liked.set(liked);
	});

	let toggle_like = Action::new(move |payload: &(String, bool)| {
		let (comment_id, like) = payload.clone();
		async move {
			if like {
				post_comment_like(comment_id).await
			} else {
				delete_comment_like(comment_id).await
			}
		}
	});

	let count_label = Signal::derive(move || format_count(like_count.get(), CountFormat::Short));

	let on_click = {
		let is_authenticated = is_authenticated;
		let toggle_like = toggle_like;
		let comment_id = comment_id_for_click;

		move |_| {
			if !is_authenticated.get_untracked() {
				show_signin_prompt.set(true);
				return;
			}

			if toggle_like.pending().get_untracked() {
				return;
			}

			let next = !is_liked.get_untracked();

			is_liked.set(next);
			if next {
				like_count.update(|count| *count += 1);
			} else {
				like_count.update(|count| *count = (*count - 1).max(0));
			}

			toggle_like.dispatch((comment_id.clone(), next));
		}
	};

	view! {
        <div class="inline-flex items-center gap-.5 -ml-2.5">
            <button
                type="button"
                class="inline-flex items-center rounded-full px-2.5 py-2.5 transition hover:bg-border"
                disabled=move || toggle_like.pending().get()
                on:click=on_click
            >
                {move || {
                    if is_liked.get() {
                        view! { <Icon class="w-4 h-4" kind=IconKind::ThumbsUpSelected /> }.into_any()
                    } else {
                        view! { <Icon class="w-4 h-4" kind=IconKind::ThumbsUp /> }.into_any()
                    }
                }}
            </button>
            <span class="text-xs font-medium text-text">{move || count_label.get()}</span>
        </div>
	}
}

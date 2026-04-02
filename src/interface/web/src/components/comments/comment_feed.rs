use leptos::prelude::*;

use crate::{
	api::{
		_dtos::comment::{CommentDto, CommentPageDto},
		comment::{get_video_comment_count, get_video_comments},
	},
	components::{comments::{CommentComposer, CommentItem}, ui::SigninPromptModal}, context::CurrentUserContext,
};

fn use_paginated_comment_feed(
	video_id: Signal<String>,
) -> (
	RwSignal<Vec<CommentDto>>,
	RwSignal<Option<String>>,
	RwSignal<bool>,
	RwSignal<bool>,
	RwSignal<bool>,
	Action<(String, String), Result<CommentPageDto, ()>>,
) {
	let comments = RwSignal::new(Vec::<CommentDto>::new());
	let next_cursor = RwSignal::new(None::<String>);
	let has_more = RwSignal::new(false);
	let initial_error = RwSignal::new(false);
	let load_more_error = RwSignal::new(false);

	let list_comments = Resource::new(
		move || video_id.get(),
		move |video_id| async move {
			get_video_comments(video_id, Some(20), None, Some("newest".to_string())).await
		},
	);

	let load_more = Action::new(move |(video_id, cursor): &(String, String)| {
		let video_id = video_id.clone();
		let cursor = cursor.clone();
		async move {
			get_video_comments(video_id, Some(20), Some(cursor), Some("newest".to_string()))
				.await
				.map_err(|_| ())
		}
	});

	Effect::new(move |_| {
		let Some(result) = list_comments.get() else {
			return;
		};

		match result {
			Ok(page) => {
				initial_error.set(false);
				comments.set(page.items);
				next_cursor.set(page.next_cursor);
				has_more.set(page.has_more);
			}
			Err(_) => {
				initial_error.set(true);
				comments.set(Vec::new());
				next_cursor.set(None);
				has_more.set(false);
			}
		}
	});

	Effect::new(move |_| {
		let Some(result) = load_more.value().get() else {
			return;
		};

		match result {
			Ok(page) => {
				load_more_error.set(false);
				comments.update(|items| items.extend(page.items));
				next_cursor.set(page.next_cursor);
				has_more.set(page.has_more);
			}
			Err(_) => load_more_error.set(true),
		}
	});

	(
		comments,
		next_cursor,
		has_more,
		initial_error,
		load_more_error,
		load_more,
	)
}

#[component]
pub fn CommentFeed(video_id: String) -> impl IntoView {
	let video_id_state = RwSignal::new(video_id);
	let current_user_ctx = use_context::<CurrentUserContext>();
	let is_authenticated = Signal::derive(move || {
		current_user_ctx
			.as_ref()
			.and_then(|ctx| ctx.current_user.get())
			.is_some()
	});
	let current_user_id = Signal::derive(move || {
		current_user_ctx
			.as_ref()
			.and_then(|ctx| ctx.current_user.get())
			.map(|user| user.id)
	});

	let show_signin_prompt = RwSignal::new(false);
	let comment_count = RwSignal::new(0_i64);
	let video_id_signal = Signal::derive(move || video_id_state.get());
	let (comments, next_cursor, has_more, initial_error, load_more_error, load_more) =
		use_paginated_comment_feed(video_id_signal);

	let count_resource = Resource::new(
		move || video_id_state.get(),
		move |video_id| async move { get_video_comment_count(video_id).await },
	);
	let refresh_count_action = Action::new(move |video_id: &String| {
		let video_id = video_id.clone();
		async move { get_video_comment_count(video_id).await }
	});

	Effect::new(move |_| {
		if let Some(Ok(count)) = count_resource.get() {
			comment_count.set(count.max(0));
		}
	});

	Effect::new(move |_| {
		if let Some(Ok(count)) = refresh_count_action.value().get() {
			comment_count.set(count.max(0));
		}
	});

	let on_comment_created = Callback::new(move |comment: CommentDto| {
		comments.update(|items| items.insert(0, comment));
		comment_count.update(|count| *count += 1);
	});
	let on_comment_deleted = Callback::new(move |deleted_comment_id: String| {
		comments.update(|items| items.retain(|comment| comment.id != deleted_comment_id));
		refresh_count_action.dispatch(video_id_state.get_untracked());
	});
	let on_reply_created = Callback::new(move |_comment: CommentDto| {
		comment_count.update(|count| *count += 1);
	});
	let is_authenticated_for_composer = is_authenticated;
	let is_authenticated_for_items = is_authenticated;
	let show_signin_for_composer = show_signin_prompt;
	let show_signin_for_items = show_signin_prompt;
	let show_signin_for_modal = show_signin_prompt;
	let on_comment_created_for_composer = on_comment_created;
	let on_reply_created_for_items = on_reply_created;

	view! {
		<section class="space-y-4">
			<div class="flex items-center justify-between">
				<h2 class="text-lg font-semibold text-text">{move || format!("{} comments", comment_count.get())}</h2>
			</div>

			<CommentComposer
				video_id=video_id_state.get_untracked()
				is_authenticated=is_authenticated_for_composer
				show_signin_prompt=show_signin_for_composer
				on_created=on_comment_created_for_composer.clone()
			/>

			<Show when=move || initial_error.get()>
				<p class="text-sm text-red-500">"Unable to load comments."</p>
			</Show>

			<div class="space-y-3">
				<For
					each=move || comments.get().into_iter()
					key=|comment| comment.id.clone()
					children=move |comment| {
						view! {
							<CommentItem
								comment=comment
								is_authenticated=is_authenticated_for_items
								current_user_id=current_user_id
								show_signin_prompt=show_signin_for_items
								on_reply_created=on_reply_created_for_items
								on_deleted=on_comment_deleted
							/>
						}
					}
				/>
			</div>

			<Show when=move || has_more.get()>
				<div class="flex justify-center">
					<button
						type="button"
						class="btn-secondary"
						disabled=move || load_more.pending().get()
						on:click=move |_| {
							if !has_more.get_untracked() || load_more.pending().get_untracked() {
								return;
							}

							if let Some(cursor) = next_cursor.get_untracked() {
								load_more.dispatch((video_id_state.get_untracked(), cursor));
							}
						}
					>
						"Load more"
					</button>
				</div>
			</Show>

			<Show when=move || load_more_error.get()>
				<p class="text-sm text-red-500">"Unable to load more comments."</p>
			</Show>

			<SigninPromptModal
				open=show_signin_for_modal
				title="Want to join the conversation ?".to_string()
				message="Sign in to continue.".to_string()
			/>
		</section>
	}
}

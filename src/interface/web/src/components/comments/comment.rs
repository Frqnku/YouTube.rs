use leptos::prelude::*;

use crate::{
	api::{
		_dtos::comment::CommentDto,
		comment::get_comment_replies,
	},
	components::{
		_helpers::format_relative_time,
		comments::{CommentComposer, comment_like::CommentLikeButton},
	},
};

#[component]
pub fn CommentItem(
	comment: CommentDto,
	is_authenticated: Signal<bool>,
	show_signin_prompt: RwSignal<bool>,
	on_reply_created: Callback<CommentDto>,
	#[prop(default = 0)] depth: u8,
) -> AnyView {
	let comment_id = comment.id;
	let comment_id_for_replies = comment_id.clone();
	let comment_id_for_like = comment_id.clone();
	let comment_id_for_reply = comment_id.clone();
	let video_id = comment.video_id;
	let comment_user = comment.user;
	let comment_user_picture = comment.user_picture;
	let comment_content = comment.content;
	let comment_like_count = comment.like_count;
	let comment_reply_count = RwSignal::new(comment.reply_count.max(0));
	let comment_liked_by_viewer = comment.liked_by_viewer;
	let posted_at = format_relative_time(&comment.created_at);
	let expanded_replies = RwSignal::new(false);
	let show_reply_composer = RwSignal::new(false);
	let replies = RwSignal::new(Vec::<CommentDto>::new());
	let replies_error = RwSignal::new(false);
	let replies_fetched = RwSignal::new(false);
	let comment_reply_count_for_toggle = comment_reply_count;

	let load_replies_action = Action::new(move |comment_id: &String| {
		let comment_id = comment_id.clone();
		async move {
			get_comment_replies(comment_id, Some(20), None, Some("newest".to_string()))
				.await
				.map(|page| page.items)
		}
	});

	Effect::new({
		let comment_id_for_replies = comment_id_for_replies.clone();
		move |_| {
			if expanded_replies.get() && !replies_fetched.get_untracked() {
				replies_fetched.set(true);
				load_replies_action.dispatch(comment_id_for_replies.clone());
			}
		}
	});

	Effect::new(move |_| {
		let Some(result) = load_replies_action.value().get() else {
			return;
		};

		match result {
			Ok(items) => {
				replies_error.set(false);
				replies.set(items);
			}
			Err(_) => {
				replies_fetched.set(false);
				replies_error.set(true);
			}
		}
	});

	let on_reply_button_click = {
		let is_authenticated = is_authenticated;
		move |_| {
			if !is_authenticated.get_untracked() {
				show_signin_prompt.set(true);
				return;
			}

			show_reply_composer.update(|value| *value = !*value);
		}
	};

	let on_reply_submitted = Callback::new(move |reply: CommentDto| {
		expanded_replies.set(true);
		show_reply_composer.set(false);
		comment_reply_count.update(|count| *count += 1);
		replies.update(|items| items.insert(0, reply.clone()));
		on_reply_created.run(reply);
	});
	let on_reply_cancel = Callback::new(move |_| show_reply_composer.set(false));

	view! {
		<article class="space-y-3 p-3">
			<div class="flex items-start gap-3">
				<img
					src=comment_user_picture.clone().unwrap_or_default()
					alt=format!("{} profile picture", comment_user)
					class="h-9 w-9 shrink-0 rounded-full bg-bg-tertiary object-cover"
				/>
				<div class="min-w-0 flex-1 space-y-1">
					<div class="flex flex-wrap items-center gap-2">
						<span class="font-medium text-text">{comment_user.clone()}</span>
						<span class="text-xs text-text-muted">{posted_at}</span>
					</div>
					<p class="whitespace-pre-line">{comment_content.clone()}</p>
					<div class="flex items-center gap-2 mt-1">
						<CommentLikeButton
							comment_id=comment_id_for_like.clone()
							initial_like_count=comment_like_count
							initial_liked_by_viewer=comment_liked_by_viewer
							is_authenticated=is_authenticated
							show_signin_prompt=show_signin_prompt
						/>
						<button
							type="button"
							class="rounded-full px-3 py-1.5 text-xs font-medium text-text-secondary transition hover:bg-border"
							on:click=on_reply_button_click
						>
							"Reply"
						</button>
						{move || {
							if comment_reply_count_for_toggle.get() > 0 {
								view! {
									<button
										type="button"
										class="rounded-full px-3 py-1.5 text-xs font-medium text-text-secondary transition hover:bg-border"
										on:click=move |_| expanded_replies.update(|value| *value = !*value)
									>
										{move || {
											if expanded_replies.get() {
												"Hide replies".to_string()
											} else {
												format!("View replies ({})", comment_reply_count.get())
											}
										}}
									</button>
								}.into_any()
							} else {
								view! { <></> }.into_any()
							}
						}}
					</div>
					<Show when=move || show_reply_composer.get()>
						<div class="pt-2">
							<CommentComposer
								video_id=video_id.clone()
								parent_id=comment_id_for_reply.clone()
								placeholder="Write a reply"
								submit_label="Reply"
								compact=true
								on_cancel=on_reply_cancel
								is_authenticated=is_authenticated
								show_signin_prompt=show_signin_prompt
								on_created=on_reply_submitted.clone()
							/>
						</div>
					</Show>
				</div>
			</div>

			<Show when=move || expanded_replies.get()>
				<div class="ml-12 space-y-3">
					<Show when=move || load_replies_action.pending().get()>
						<p class="text-xs text-text-muted">"Loading replies..."</p>
					</Show>
					<Show when=move || replies_error.get()>
						<p class="text-xs text-red-500">"Unable to load replies."</p>
					</Show>
					<Show when=move || !load_replies_action.pending().get() && !replies_error.get() && replies.with(|r| r.is_empty())>
						<p class="text-xs text-text-muted">"No replies yet."</p>
					</Show>
					<For
						each=move || replies.get().into_iter()
						key=|reply| reply.id.clone()
						children=move |reply| {
							view! {
								<CommentItem
									comment=reply
									is_authenticated=is_authenticated
									show_signin_prompt=show_signin_prompt
									on_reply_created=on_reply_created
									depth=depth + 1
								/>
							}
						}
					/>
				</div>
			</Show>
		</article>
	}.into_any()
}

use leptos::prelude::*;

use crate::{
	api::{
		_dtos::comment::CommentDto,
		comment::post_comment,
	},
	app::CurrentUserContext,
};

#[component]
pub fn CommentComposer(
	video_id: String,
	is_authenticated: Signal<bool>,
	show_signin_prompt: RwSignal<bool>,
	on_created: Callback<CommentDto>,
	#[prop(optional)] on_cancel: Option<Callback<()>>,
	#[prop(optional)] parent_id: Option<String>,
	#[prop(into, optional)] placeholder: String,
	#[prop(into, optional)] submit_label: String,
	#[prop(optional)] compact: bool,
) -> impl IntoView {
	let current_user_ctx = use_context::<CurrentUserContext>();
	let current_user = move || current_user_ctx.as_ref().and_then(|ctx| ctx.current_user.get());
	let content = RwSignal::new(String::new());
	let submit_action = Action::new(move |payload: &(String, String, Option<String>)| {
		let (video_id, content, parent_id) = payload.clone();
		async move { post_comment(video_id, content, parent_id).await }
	});

	let placeholder_text = if placeholder.is_empty() {
		"Add a comment".to_string()
	} else {
		placeholder
	};
	let submit_text = if submit_label.is_empty() {
		"Comment".to_string()
	} else {
		submit_label
	};

	let on_submit = {
		let video_id = video_id.clone();
		let parent_id = parent_id.clone();
		move |_| {
			if !is_authenticated.get_untracked() {
				show_signin_prompt.set(true);
				return;
			}

			if submit_action.pending().get_untracked() {
				return;
			}

			let trimmed = content.get_untracked().trim().to_string();
			if trimmed.is_empty() {
				return;
			}

			submit_action.dispatch((video_id.clone(), trimmed, parent_id.clone()));
		}
	};

	Effect::new(move |_| {
		let Some(result) = submit_action.value().get() else {
			return;
		};

		if let Ok(comment) = result {
			on_created.run(comment);
			content.set(String::new());
		}
	});

	let container_class = if compact {
		"flex gap-3"
	} else {
		"flex gap-3"
	};
	let actions_class = if compact {
		"mt-2 flex justify-end gap-3"
	} else {
		"mt-2 flex justify-end gap-4"
	};
	let input_class = if compact {
		"w-full border-b bg-bg py-2 text-sm text-text outline-none focus:border-accent"
	} else {
		"w-full border-b bg-bg py-2 text-sm text-text outline-none focus:border-accent"
	};
	let on_cancel_click = move |_| {
		content.set(String::new());
		if let Some(on_cancel) = on_cancel.as_ref() {
			on_cancel.run(());
		}
	};

	view! {
		<div class=container_class>
			<img
				src=move || current_user()
					.and_then(|user| user.profile_picture.clone())
					.unwrap_or("https://t3.ftcdn.net/jpg/00/64/67/80/360_F_64678017_zUpiZFjj04cnLri7oADnyMH0XBYyQghG.jpg".to_string())
				alt="Profile picture"
				class="h-8 w-8 rounded-full bg-bg-tertiary object-cover"
			/>
			<div class="w-full">
				<input
					class=input_class
					placeholder=placeholder_text
					prop:value=move || content.get()
					on:input=move |event| content.set(event_target_value(&event))
				/>
				<div class=actions_class>
					<button
						type="button"
						class="text-text-primary transition hover:text-text-disabled"
						disabled=move || submit_action.pending().get()
						on:click=on_cancel_click
					>
						"Cancel"
					</button>
					<button
						type="button"
						class="btn-primary"
						disabled=move || submit_action.pending().get() || content.get().trim().is_empty()
						on:click=on_submit
					>
						{submit_text}
					</button>
				</div>
			</div>
		</div>
	}
}
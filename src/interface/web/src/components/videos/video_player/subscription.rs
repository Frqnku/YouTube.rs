use leptos::prelude::*;

use crate::{
	api::subscription::{
		delete_subscription,
		get_subscription_status,
		post_subscription,
	},
	components::{_helpers::{CountFormat, format_count}, ui::icons::{Icon, IconKind}}, context::SubscriptionsContext,
};

#[component]
pub fn Channel(
	channel_name: String,
	subscriber_count: RwSignal<usize>,
	subscriber_count_loaded: Signal<bool>,
) -> impl IntoView {
	let subscriber_count_label = Signal::derive(move || {
		format!(
			"{} subscribers",
			format_count(subscriber_count.get() as i64, CountFormat::Short)
		)
	});

	view! {
		<div>
			<p class="font-medium text-text">{channel_name}</p>
			<Show
				when=move || subscriber_count_loaded.get()
				fallback=move || {
					view! {
						<span
							aria-hidden="true"
							class="mt-1 block h-4 w-24 animate-pulse rounded bg-bg-tertiary"
						></span>
					}
				}
			>
				<p class="text-sm text-text-secondary">{move || subscriber_count_label.get()}</p>
			</Show>
		</div>
	}
}

#[component]
pub fn SubscribeButton(
	channel_id: String,
	is_authenticated: Signal<bool>,
	show_signin_prompt: RwSignal<bool>,
	subscriber_count: RwSignal<usize>,
	prompt_title: RwSignal<String>,
	prompt_message: RwSignal<String>,
) -> impl IntoView {
	let is_subscribed = RwSignal::new(false);
	let channel_id_for_status = channel_id.clone();
	let channel_id_for_click = channel_id.clone();

	let status_resource = Resource::new(
		move || (is_authenticated.get(), channel_id_for_status.clone()),
		move |(authed, channel_id)| async move {
			if !authed {
				Ok(false)
			} else {
				get_subscription_status(channel_id).await
			}
		},
	);

	Effect::new(move |_| {
		if let Some(Ok(status)) = status_resource.get() {
			is_subscribed.set(status);
		}
	});

	let toggle_subscription = Action::new(move |payload: &(String, bool)| {
		let (channel_id, next_subscribed) = payload.clone();
		async move {
			if next_subscribed {
				post_subscription(channel_id).await
			} else {
				delete_subscription(channel_id).await
			}
		}
	});

	Effect::new(move |_| {
		let Some(result) = toggle_subscription.value().get() else {
			return;
		};

		if result.is_ok() {
			// Update subscriber count locally: +1 if subscribed, -1 if unsubscribed
			let is_sub = is_subscribed.get_untracked();
			let current_count = subscriber_count.get_untracked();
			subscriber_count.set(if is_sub {
				current_count.saturating_add(1)
			} else {
				current_count.saturating_sub(1)
			});
			// Notify sidebar to refetch subscriptions list
			if let Some(ctx) = use_context::<SubscriptionsContext>() {
				ctx.refetch_trigger.update(|n| *n += 1);
			}
		} else {
			let currently_subscribed = is_subscribed.get_untracked();
			is_subscribed.set(!currently_subscribed);
		}
	});

	let button_label = Signal::derive(move || {
		if is_subscribed.get() {
			"Subscribed".to_string()
		} else {
			"Subscribe".to_string()
		}
	});
	let button_class = Signal::derive(move || {
		if is_subscribed.get() {
			"btn-secondary"
		} else {
			"btn-primary"
		}
	});

	view! {
		<button
			type="button"
			class=move || button_class.get()
			disabled=move || toggle_subscription.pending().get()
			on:click=move |_| {
				if !is_authenticated.get_untracked() {
					prompt_title.set("Want to subscribe ?".to_string());
					prompt_message.set("Sign in to subscribe to this channel.".to_string());
					show_signin_prompt.set(true);
					return;
				}

				if toggle_subscription.pending().get_untracked() {
					return;
				}

				let next_subscribed = !is_subscribed.get_untracked();
				is_subscribed.set(next_subscribed);
				toggle_subscription.dispatch((channel_id_for_click.clone(), next_subscribed));
			}
		>
			<span class="inline-flex items-center gap-2">
				<Show when=move || is_subscribed.get()>
					<Icon kind=IconKind::BellActive class="h-4 w-4 text-text-secondary" />
				</Show>
				<span>{move || button_label.get()}</span>
			</span>
		</button>
	}
}

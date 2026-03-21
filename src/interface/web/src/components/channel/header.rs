use std::sync::Arc;

use leptos::prelude::*;

use crate::{
	api::_dtos::channel::ChannelDataDto,
	api::subscription::{
		delete_subscription,
		get_subscription_status,
		post_subscription,
	},
	app::{CurrentUserContext, SubscriptionsContext},
	components::_helpers::{CountFormat, format_count},
};

#[component]
fn ChannelBanner(banner_url: Option<String>) -> impl IntoView {
	let banner_src = Arc::new(banner_url);
	let has_banner = banner_src.is_some();

	view! {
		<Show when=move || has_banner>
			<div class="overflow-hidden rounded-2xl bg-bg-secondary">
				<img
					src={
						let banner_src = Arc::clone(&banner_src);
						move || banner_src.as_ref().clone().unwrap_or_default()
					}
					alt="Channel banner"
					class="h-40 w-full object-cover md:h-56"
				/>
			</div>
		</Show>
	}
}

#[component]
pub fn ChannelHeader(
	channel: ChannelDataDto,
) -> impl IntoView {
	let current_user_ctx = use_context::<CurrentUserContext>();
	let subscriptions_ctx = use_context::<SubscriptionsContext>();
	let is_authenticated = Signal::derive(move || {
		current_user_ctx
			.as_ref()
			.and_then(|ctx| ctx.current_user.get())
			.is_some()
	});

	let channel_id = channel.id.clone();
	let channel_id_for_status = channel_id.clone();
	let channel_id_for_click = channel_id;

	let is_subscribed = RwSignal::new(false);
	let subscriber_count_delta = RwSignal::new(0i64);
	let show_signin_prompt = RwSignal::new(false);

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
			if is_subscribed.get_untracked() {
				subscriber_count_delta.update(|delta| *delta += 1);
			} else {
				subscriber_count_delta.update(|delta| *delta -= 1);
			}

			if let Some(ctx) = subscriptions_ctx {
				ctx.refetch_trigger.update(|n| *n += 1);
			}
		} else {
			is_subscribed.update(|value| *value = !*value);
		}
	});

	let on_subscribe_click = Callback::new(move |_| {
		if !is_authenticated.get_untracked() {
			show_signin_prompt.set(true);
			return;
		}

		if toggle_subscription.pending().get_untracked() {
			return;
		}

		let next_subscribed = !is_subscribed.get_untracked();
		is_subscribed.set(next_subscribed);
		toggle_subscription.dispatch((channel_id_for_click.clone(), next_subscribed));
	});

	let subscribe_pending = Signal::derive(move || toggle_subscription.pending().get());

	let profile_picture_alt = format!("{} profile picture", channel.name);
	let subscribe_label = Signal::derive(move || {
		if is_subscribed.get() {
			"Subscribed".to_string()
		} else {
			"Subscribe".to_string()
		}
	});
	let subscribe_class = Signal::derive(move || {
		if is_subscribed.get() {
			"btn-secondary self-start shrink-0".to_string()
		} else {
			"btn-primary self-start shrink-0".to_string()
		}
	});
	let base_subscriber_count = channel.subscriber_count as i64;
	let subscriber_count_label = Signal::derive(move || {
		let effective_count = (base_subscriber_count + subscriber_count_delta.get()).max(0);
		format!(
			"{} subscribers",
			format_count(effective_count, CountFormat::Short),
		)
	});
	let video_count_label = format!(
		"{} videos",
		format_count(channel.video_count as i64, CountFormat::Short)
	);
	let profile_picture_url = channel.profile_picture.unwrap_or_default();
	let banner_url = channel.banner;
	let channel_name = channel.name;
	let channel_description = channel.description;

	view! {
		<ChannelBanner banner_url=banner_url />

		<div class="mt-8 flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
			<div class="flex items-center gap-4">
				<img
					src=profile_picture_url
					alt=profile_picture_alt
					class="h-24 w-24 rounded-full bg-bg-secondary object-cover md:h-38 md:w-38"
				/>
				<div class="flex flex-col gap-2">
					<h1 class="text-2xl font-semibold text-text md:text-2xl">{channel_name.clone()}</h1>
					<p class="text-md text-text-secondary">
						{move || subscriber_count_label.get()}
						<span class="mx-1">"•"</span>
						{video_count_label.clone()}
					</p>
					<p class="text-md text-text-secondary">{channel_description.clone()}</p>
					<button
						type="button"
						class=move || subscribe_class.get()
						disabled=move || subscribe_pending.get()
						on:click=move |event| on_subscribe_click.run(event)
					>
						{move || subscribe_label.get()}
					</button>
				</div>
			</div>
		</div>

		<Show when=move || show_signin_prompt.get()>
			<div class="mt-3 rounded-xl bg-bg-secondary p-3 text-sm text-text-secondary">
				"Sign in to subscribe to this channel. "
				<a href="/signin" class="font-medium text-text underline">"Sign in"</a>
			</div>
		</Show>
	}
}
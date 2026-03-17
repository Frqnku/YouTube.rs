use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

use crate::{
	api::{
		_dtos::video::VideoCardPage,
		channel::get_channel_data,
		subscription::{
			delete_subscription,
			get_subscription_status,
			post_subscription,
		},
		video::get_channel_videos,
	},
	app::{CurrentUserContext, SubscriptionsContext},
	components::{
		_helpers::{CountFormat, format_count},
		ui::{LineDivider, Loader, NotFound},
		videos::{
			video_feed::{ResponsiveVideoCardSkeletons, use_paginated_feed},
			VideoCard,
		},
	},
};

const CHANNEL_PAGE_SIZE: u32 = 12;
const DEFAULT_BANNER_URL: &str = "https://images.unsplash.com/photo-1489515217757-5fd1be406fef?auto=format&fit=crop&w=1600&q=80";

#[component]
pub fn ChannelPage() -> impl IntoView {
	let query_map = use_query_map();
	let channel_id = Signal::derive(move || {
		query_map.with(|qm| qm.get("id").map(|value| value.to_string()))
	});

	let current_user_ctx = use_context::<CurrentUserContext>();
	let subscriptions_ctx = use_context::<SubscriptionsContext>();
	let is_authenticated = Signal::derive(move || {
		current_user_ctx
			.as_ref()
			.and_then(|ctx| ctx.current_user.get())
			.is_some()
	});

	let is_subscribed = RwSignal::new(false);
	let subscriber_count_delta = RwSignal::new(0i64);
	let show_signin_prompt = RwSignal::new(false);

	let channel_resource = Resource::new(
		move || channel_id.get(),
		move |maybe_id| async move {
			match maybe_id {
				Some(id) => get_channel_data(id).await,
				None => Ok(None),
			}
		},
	);

	Effect::new(move |_| {
		let _ = channel_id.get();
		subscriber_count_delta.set(0);
	});

	let status_resource = Resource::new(
		move || (is_authenticated.get(), channel_id.get()),
		move |(authed, maybe_channel_id)| async move {
			if !authed {
				Ok(false)
			} else if let Some(channel_id) = maybe_channel_id {
				get_subscription_status(channel_id).await
			} else {
				Ok(false)
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

	let on_subscribe_click = move |_| {
		if !is_authenticated.get_untracked() {
			show_signin_prompt.set(true);
			return;
		}

		if toggle_subscription.pending().get_untracked() {
			return;
		}

		let Some(channel_id) = channel_id.get_untracked() else {
			return;
		};

		let next_subscribed = !is_subscribed.get_untracked();
		is_subscribed.set(next_subscribed);
		toggle_subscription.dispatch((channel_id, next_subscribed));
	};

	let (
		videos,
		_next_cursor,
		_has_more,
		initial_error,
		load_more_error,
		load_more,
	) = use_paginated_feed(channel_id, |maybe_channel_id, cursor| async move {
		match maybe_channel_id {
			Some(id) => get_channel_videos(id, Some(CHANNEL_PAGE_SIZE), cursor)
				.await
				.map_err(|_| ()),
			None => Ok(VideoCardPage::new(Vec::new(), None, false)),
		}
	});

	view! {
		<main class="min-h-[calc(100dvh-3.5rem)] bg-bg px-4 py-6 md:px-6">
			<Suspense fallback=move || view! { <Loader /> }>
				{move || {
					channel_resource.get().map(|channel_result| match channel_result {
						Ok(Some(channel)) => {
							let subscribe_label = Signal::derive(move || {
								if is_subscribed.get() {
									"Subscribed".to_string()
								} else {
									"Subscribe".to_string()
								}
							});

							let subscribe_class = Signal::derive(move || {
								if is_subscribed.get() {
									"btn-secondary"
								} else {
									"btn-primary"
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

							view! {
								<section class="mx-auto w-full max-w-7xl">
									<div class="overflow-hidden rounded-2xl bg-bg-secondary">
										<img
											src=DEFAULT_BANNER_URL
											alt="Channel banner"
											class="h-40 w-full object-cover md:h-56"
										/>
									</div>

									<div class="mt-4 flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
										<div class="flex items-center gap-4">
											<img
												src=channel.profile_picture.unwrap_or_default()
												alt=format!("{} profile picture", channel.name)
												class="h-20 w-20 rounded-full bg-bg-secondary object-cover md:h-24 md:w-24"
											/>
											<div>
												<h1 class="text-xl font-semibold text-text md:text-2xl">{channel.name}</h1>
												<p class="mt-1 text-sm text-text-secondary">
													{move || subscriber_count_label.get()}
													<span class="mx-1">"•"</span>
													{video_count_label.clone()}
												</p>
											</div>
										</div>

										<button
											type="button"
											class=move || subscribe_class.get()
											disabled=move || toggle_subscription.pending().get()
											on:click=on_subscribe_click
										>
											{move || subscribe_label.get()}
										</button>
									</div>

									<Show when=move || show_signin_prompt.get()>
										<div class="mt-3 rounded-xl bg-bg-secondary p-3 text-sm text-text-secondary">
											"Sign in to subscribe to this channel. "
											<a href="/signin" class="font-medium text-text underline">"Sign in"</a>
										</div>
									</Show>

									<LineDivider margin="my-6".to_string() />

									<section class="grid grid-cols-1 gap-6 lg:grid-cols-2 2xl:grid-cols-3">
										<Suspense fallback=move || view! { <ResponsiveVideoCardSkeletons /> }.into_any()>
											{move || {
												if initial_error.get() {
													return view! {
														<article class="col-span-full rounded-xl bg-bg-secondary p-4 text-sm text-text-secondary">
															"Unable to load channel videos right now. Please try again later."
														</article>
													}
													.into_any()
													.into_view();
												}

												if videos.get().is_empty() {
													return view! {
														<article class="col-span-full rounded-xl bg-bg-secondary p-4 text-sm text-text-secondary">
															"No videos yet."
														</article>
													}
													.into_any()
													.into_view();
												}

												view! {
													<For
														each=move || videos.get()
														key=|video| video.id.clone()
														children=move |video| view! { <VideoCard video=video /> }
													/>
												}
												.into_any()
												.into_view()
											}}
										</Suspense>

										<Show when=move || load_more.pending().get()>
											<ResponsiveVideoCardSkeletons />
										</Show>
									</section>

									<Show when=move || load_more_error.get()>
										<div class="pb-5 text-center text-sm text-text-secondary">
											"Couldn't load more videos. Keep scrolling to retry."
										</div>
									</Show>
								</section>
							}
							.into_any()
						}
						Ok(None) => view! { <NotFound message="Channel not found".to_string() /> }.into_any(),
						Err(error) => view! { <NotFound message=error.message /> }.into_any(),
					})
				}}
			</Suspense>
		</main>
	}
}

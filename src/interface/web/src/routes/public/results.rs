use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

use crate::api::_dtos::video::VideoCardDto;
use crate::api::video::get_videos_by_search;
#[cfg(target_arch = "wasm32")]
use crate::components::_helpers::is_near_bottom_of_page;
use crate::components::ui::{Loader, NotFound};
use crate::components::video::{VideoCard, VideoCardSkeleton};

const RESULT_PAGE_SIZE: u32 = 12;

#[component]
fn ResponsiveVideoCardSkeletons() -> impl IntoView {
	view! {
		<For
			each=move || 0..6
			key=|index| *index
			children=move |index| {
				let visibility_class = match index {
					0 | 1 => "block",
					2 | 3 => "hidden lg:block",
					_ => "hidden 2xl:block",
				};

				view! {
					<div class=visibility_class>
						<VideoCardSkeleton />
					</div>
				}
			}
		/>
	}
}

#[component]
pub fn ResultsPage() -> impl IntoView {
	let query_map = use_query_map();
	let search_query = Signal::derive(move || {
		query_map.with(|query| {
			query
				.get("search")
				.map(|value| value.to_string())
				.filter(|value| !value.trim().is_empty())
		})
	});

	let videos = RwSignal::new(Vec::<VideoCardDto>::new());
	let next_cursor = RwSignal::new(None::<String>);
	let has_more = RwSignal::new(false);
	let load_more_error = RwSignal::new(false);

	let search_results = Resource::new(
		move || search_query.get(),
		move |query| async move {
			match query {
				Some(query) => get_videos_by_search(query, Some(RESULT_PAGE_SIZE), None).await,
				None => Ok(crate::api::_dtos::video::VideoCardPage::new(Vec::new(), None, false)),
			}
		},
	);

	let load_more = Action::new(move |(query, cursor): &(String, String)| {
		let query = query.clone();
		let cursor = cursor.clone();
		async move { get_videos_by_search(query, Some(RESULT_PAGE_SIZE), Some(cursor)).await }
	});

	Effect::new(move |_| {
		let Some(result) = search_results.get() else {
			return;
		};

		match result {
			Ok(page) => {
				videos.set(page.items);
				next_cursor.set(page.next_cursor);
				has_more.set(page.has_more);
			}
			Err(_) => {
				videos.set(Vec::new());
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
				videos.update(|items| items.extend(page.items));
				next_cursor.set(page.next_cursor);
				has_more.set(page.has_more);
			}
			Err(_) => {
				load_more_error.set(true);
			}
		}
	});

	#[cfg(target_arch = "wasm32")]
	{
		let window_scroll_listener = window_event_listener(leptos::ev::scroll, move |_| {
			if !is_near_bottom_of_page() {
				return;
			}

			if !has_more.get_untracked() || load_more.pending().get_untracked() {
				return;
			}

			let Some(query) = search_query.get_untracked() else {
				return;
			};

			if let Some(cursor) = next_cursor.get_untracked() {
				load_more_error.set(false);
				load_more.dispatch((query, cursor));
			}
		});

		StoredValue::new(window_scroll_listener);
	}

	view! {
		<div class="min-h-[calc(100dvh-3.5rem)] bg-bg px-4 py-5 md:px-6">
			<div class="mb-5">
				<h1 class="text-lg font-semibold text-text md:text-xl">"Search results"</h1>
				<Show when=move || search_query.get().is_some()>
					<p class="mt-1 text-sm text-text-secondary">
						{move || format!("Results for \"{}\"", search_query.get().unwrap_or_default())}
					</p>
				</Show>
			</div>

			<section
				class=move || {
						if videos.get().is_empty() {
							"grid grid-cols-1 gap-6"
						} else {
							"grid grid-cols-1 gap-6 lg:grid-cols-2 2xl:grid-cols-3"
						}
					}
				data-section="result-grid"
			>
				<Suspense
					fallback=move || {
						view! { <ResponsiveVideoCardSkeletons /> }.into_any()
					}
				>
					{move || {
						match search_results.get() {
							None => view! {}.into_any().into_view(),
							Some(Err(_)) => view! {
								<article class="col-span-full rounded-xl border border-border bg-bg-secondary p-4 text-sm text-text-secondary">
									"Unable to load search results right now. Please try again later."
								</article>
							}
							.into_any()
							.into_view(),
							Some(Ok(page))
								if page.items.is_empty() || search_query.get().is_none() =>
							{
								view! { <NotFound message="No results found".to_string() /> }
									.into_any()
									.into_view()
							}
							Some(Ok(_)) => view! {
								<For
									each=move || videos.get()
									key=|video| video.id.clone()
									children=move |video| {
										view! { <VideoCard video=video /> }
									}
								/>
							}
							.into_any()
							.into_view(),
						}
					}}
				</Suspense>
			</section>

			<Show when=move || load_more.pending().get()>
				<ResponsiveVideoCardSkeletons />
				<Loader />
			</Show>

			<Show when=move || load_more_error.get()>
				<div class="pb-5 text-center text-sm text-text-secondary">
					"Couldn't load more results. Keep scrolling to retry."
				</div>
			</Show>
		</div>
	}
}

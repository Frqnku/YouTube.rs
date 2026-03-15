use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

use crate::api::_dtos::video::VideoCardPage;
use crate::api::video::get_videos_by_search;
use crate::components::ui::{Loader, NotFound};
use crate::components::videos::video_feed::{ResponsiveVideoCardSkeletons, use_paginated_feed};
use crate::components::videos::VideoCard;

const RESULT_PAGE_SIZE: u32 = 6;



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

	let (
		videos,
		_next_cursor,
		_has_more,
		initial_error,
		load_more_error,
		load_more,
	) = use_paginated_feed(search_query, |query, cursor| async move {
		match query {
			Some(query) => get_videos_by_search(query, Some(RESULT_PAGE_SIZE), cursor)
				.await
				.map_err(|_| ()),
			None => Ok(VideoCardPage::new(Vec::new(), None, false)),
		}
	});

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
						if initial_error.get() {
							return view! {
								<article class="col-span-full rounded-xl bg-bg-secondary p-4 text-sm text-text-secondary">
									"Unable to load search results right now. Please try again later."
								</article>
							}
								.into_any()
								.into_view();
						}

						if videos.get().is_empty() || search_query.get().is_none() {
							view! { <NotFound message="No results found".to_string() /> }
								.into_any()
								.into_view()
						} else {
							view! {
								<For
									each=move || videos.get()
									key=|video| video.id.clone()
									children=move |video| {
										view! { <VideoCard video=video /> }
									}
								/>
							}
								.into_any()
								.into_view()
						}
					}}
				</Suspense>
				<Show when=move || load_more.pending().get()>
					<ResponsiveVideoCardSkeletons />
				</Show>
			</section>


			<Show when=move || load_more_error.get()>
				<div class="pb-5 text-center text-sm text-text-secondary">
					"Couldn't load more results. Keep scrolling to retry."
				</div>
			</Show>
		</div>
	}
}

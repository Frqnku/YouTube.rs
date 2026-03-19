use leptos::prelude::*;

use crate::api::_dtos::video::VideoCardPage;
use crate::api::video::get_channel_videos;
use crate::components::videos::VideoCard;
use crate::components::videos::video_feed::{
	ResponsiveVideoCardSkeletons,
	use_paginated_feed,
};

const CHANNEL_PAGE_SIZE: u32 = 12;

#[component]
pub fn ChannelVideosSection(
    channel_id: Signal<Option<String>>,
) -> impl IntoView {
	let (videos, _next_cursor, _has_more, initial_error, load_more_error, load_more) =
		use_paginated_feed(channel_id, |maybe_channel_id, cursor| async move {
			match maybe_channel_id {
				Some(id) => get_channel_videos(id, Some(CHANNEL_PAGE_SIZE), cursor)
					.await
					.map_err(|_| ()),
				None => Ok(VideoCardPage::new(Vec::new(), None, false)),
			}
		});

	view! {
		<>
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
		</>
	}
}
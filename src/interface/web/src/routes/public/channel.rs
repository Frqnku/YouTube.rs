use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

use crate::{
	api::{
		channel::get_channel_data,
	},
	components::{
		channel::{ChannelHeader, ChannelVideosSection},
		ui::{LineDivider, Loader, NotFound},
	},
};

#[component]
pub fn ChannelPage() -> impl IntoView {
	let query_map = use_query_map();
	let channel_id = Signal::derive(move || {
		query_map.with(|qm| qm.get("id").map(|value| value.to_string()))
	});

	let channel_resource = Resource::new(
		move || channel_id.get(),
		move |maybe_id| async move {
			match maybe_id {
				Some(id) => get_channel_data(id).await,
				None => Ok(None),
			}
		},
	);

	view! {
		<main class="min-h-[calc(100dvh-3.5rem)] bg-bg px-4 py-6 md:px-6">
			<Suspense fallback=move || view! { <Loader /> }>
				{move || {
					channel_resource.get().map(|channel_result| match channel_result {
						Ok(Some(channel)) => {
							view! {
								<section class="mx-auto w-full max-w-7xl">
									<ChannelHeader channel=channel />

									<LineDivider margin="my-6".to_string() />

									<ChannelVideosSection channel_id=channel_id />
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

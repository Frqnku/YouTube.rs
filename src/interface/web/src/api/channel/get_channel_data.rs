use leptos::prelude::*;

#[cfg(feature = "ssr")]
use application::queries::GetChannelData as GetChannelDataUsecase;
#[cfg(feature = "ssr")]
use infrastructure::repositories::PgChannelRepository;

use crate::api::{
	_dtos::channel::ChannelDataDto,
	_errors::AppServerError,
};
#[cfg(feature = "ssr")]
use crate::api::_errors::OptionExt;

#[server]
pub async fn get_channel_data(channel_id: String) -> Result<Option<ChannelDataDto>, AppServerError> {
	let pool = use_context::<sqlx::PgPool>()
		.require_context("Missing pool")?;

	let repository = PgChannelRepository::new(&pool);
	let query = GetChannelDataUsecase {
		channel_repository: &repository,
	};

	let channel = query
		.execute(channel_id)
		.await
		.map_err(AppServerError::from)?;

	Ok(channel.map(ChannelDataDto::from))
}

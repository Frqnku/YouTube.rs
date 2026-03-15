use leptos::prelude::*;

#[cfg(feature = "ssr")]
use application::commands::{RegisterVideoView, UpdateWatchedSeconds as UpdateWatchedSecondsUsecase};
#[cfg(feature = "ssr")]
use infrastructure::repositories::PgVideoViewRepository;

use crate::api::_errors::AppServerError;
#[cfg(feature = "ssr")]
use crate::{api::_errors::OptionExt, app::{ClientRequestMeta, CurrentUser}};

#[server]
pub async fn post_video_view(video_id: String) -> Result<(), AppServerError> {
	let pool = use_context::<sqlx::PgPool>()
		.require_context("Missing pool")?;

	let current_user = use_context::<CurrentUser>();
	let client_meta = use_context::<ClientRequestMeta>();

	let repository = PgVideoViewRepository::new(&pool);
	let command = RegisterVideoView {
		view_repository: &repository,
	};

	command
		.execute(
			video_id,
			current_user.map(|user| user.id),
			client_meta.and_then(|meta| meta.ip_address),
		)
		.await
		.map_err(AppServerError::from)?;

	Ok(())
}

#[server]
pub async fn update_watched_seconds(video_id: String, watched_seconds: u32) -> Result<(), AppServerError> {
	let pool = use_context::<sqlx::PgPool>()
		.require_context("Missing pool")?;

	let current_user = use_context::<CurrentUser>()
		.require_context("Missing current user")?;

	let repository = PgVideoViewRepository::new(&pool);
	let command = UpdateWatchedSecondsUsecase {
		view_repository: &repository,
	};

	command
		.execute(
			video_id,
			current_user.id,
			watched_seconds,
		)
		.await
		.map_err(AppServerError::from)?;

	Ok(())
}

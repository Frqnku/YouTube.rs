use leptos::prelude::*;

#[cfg(feature = "ssr")]
use application::commands::RegisterVideoView;
#[cfg(feature = "ssr")]
use infrastructure::repositories::PgVideoRepository;

use crate::api::_errors::AppServerError;
#[cfg(feature = "ssr")]
use crate::{api::_errors::OptionExt, app::{ClientRequestMeta, CurrentUser}};

#[server]
pub async fn post_video_view(video_id: String) -> Result<(), AppServerError> {
	let pool = use_context::<sqlx::PgPool>()
		.require_context("Missing pool")?;

	let current_user = use_context::<CurrentUser>();
	let client_meta = use_context::<ClientRequestMeta>();

	let repository = PgVideoRepository::new(&pool);
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

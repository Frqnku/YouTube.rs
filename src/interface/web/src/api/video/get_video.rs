use leptos::prelude::*;

#[cfg(feature = "ssr")]
use domain::_shared::DomainError;
#[cfg(feature = "ssr")]
use application::queries::GetVideoById;
#[cfg(feature = "ssr")]
use infrastructure::repositories::PgVideoRepository;

use crate::api::{_dtos::video::VideoPlayer, _errors::AppServerError};
#[cfg(feature = "ssr")]
use crate::api::_errors::OptionExt;

#[server]
pub async fn get_video(id: String) -> Result<VideoPlayer, AppServerError> {
	let pool = use_context::<sqlx::PgPool>()
		.require_context("Missing pool")?;
	let viewer_user_id = use_context::<crate::app::CurrentUser>().map(|user| user.id);

	let repository = PgVideoRepository::new(&pool);
	let query = GetVideoById {
		video_repository: &repository,
	};

    let video = query.execute(id, viewer_user_id)
        .await
		.map_err(AppServerError::from)?
        .ok_or_else(|| AppServerError::from(DomainError::VideoNotFound))?;

    Ok(video.into())
}
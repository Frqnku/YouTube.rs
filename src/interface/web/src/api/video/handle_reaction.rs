use leptos::prelude::*;

#[cfg(feature = "ssr")]
use application::commands::{
	AddVideoDislike,
	AddVideoLike,
	GetVideoReactionStatus as GetVideoReactionStatusUseCase,
	RemoveVideoDislike,
	RemoveVideoLike,
};
#[cfg(feature = "ssr")]
use domain::_shared::DomainError;
#[cfg(feature = "ssr")]
use infrastructure::repositories::PgVideoRepository;

use crate::api::_errors::AppServerError;
#[cfg(feature = "ssr")]
use crate::{api::_errors::OptionExt, app::CurrentUser};

#[server]
pub async fn get_video_reaction(video_id: String) -> Result<(bool, bool), AppServerError> {
	let pool = use_context::<sqlx::PgPool>()
		.require_context("Missing pool")?;

	let current_user = use_context::<CurrentUser>()
		.ok_or_else(|| AppServerError::from(DomainError::Unauthorized))?;

	let repository = PgVideoRepository::new(&pool);
	let query = GetVideoReactionStatusUseCase {
		reaction_repository: &repository,
	};

	let status = query
		.execute(current_user.id, video_id)
		.await
		.map_err(AppServerError::from)?;

	Ok(status)
}

#[server]
pub async fn post_video_like(video_id: String) -> Result<(), AppServerError> {
	let pool = use_context::<sqlx::PgPool>()
		.require_context("Missing pool")?;

	let current_user = use_context::<CurrentUser>()
		.ok_or_else(|| AppServerError::from(DomainError::Unauthorized))?;

	let repository = PgVideoRepository::new(&pool);
	let command = AddVideoLike {
		reaction_repository: &repository,
	};

	command
		.execute(current_user.id, video_id)
		.await
		.map_err(AppServerError::from)?;

	Ok(())
}

#[server]
pub async fn delete_video_like(video_id: String) -> Result<(), AppServerError> {
	let pool = use_context::<sqlx::PgPool>()
		.require_context("Missing pool")?;

	let current_user = use_context::<CurrentUser>()
		.ok_or_else(|| AppServerError::from(DomainError::Unauthorized))?;

	let repository = PgVideoRepository::new(&pool);
	let command = RemoveVideoLike {
		reaction_repository: &repository,
	};

	command
		.execute(current_user.id, video_id)
		.await
		.map_err(AppServerError::from)?;

	Ok(())
}

#[server]
pub async fn post_video_dislike(video_id: String) -> Result<(), AppServerError> {
	let pool = use_context::<sqlx::PgPool>()
		.require_context("Missing pool")?;

	let current_user = use_context::<CurrentUser>()
		.ok_or_else(|| AppServerError::from(DomainError::Unauthorized))?;

	let repository = PgVideoRepository::new(&pool);
	let command = AddVideoDislike {
		reaction_repository: &repository,
	};

	command
		.execute(current_user.id, video_id)
		.await
		.map_err(AppServerError::from)?;

	Ok(())
}

#[server]
pub async fn delete_video_dislike(video_id: String) -> Result<(), AppServerError> {
	let pool = use_context::<sqlx::PgPool>()
		.require_context("Missing pool")?;

	let current_user = use_context::<CurrentUser>()
		.ok_or_else(|| AppServerError::from(DomainError::Unauthorized))?;

	let repository = PgVideoRepository::new(&pool);
	let command = RemoveVideoDislike {
		reaction_repository: &repository,
	};

	command
		.execute(current_user.id, video_id)
		.await
		.map_err(AppServerError::from)?;

	Ok(())
}

use leptos::prelude::*;

#[cfg(feature = "ssr")]
use application::queries::ListVideos;
#[cfg(feature = "ssr")]
use domain::video::DEFAULT_PAGE_LIMIT;
#[cfg(feature = "ssr")]
use infrastructure::repositories::PgVideoRepository;

use crate::api::{_dtos::video::VideoCardPage, _errors::AppServerError};
#[cfg(feature = "ssr")]
use crate::api::_errors::OptionExt;

#[server]
pub async fn get_newest_videos(
	limit: Option<u32>,
	cursor: Option<String>,
) -> Result<VideoCardPage, AppServerError> {
	let pool = use_context::<sqlx::PgPool>()
		.require_context("Missing pool")?;

	let repository = PgVideoRepository::new(&pool);
	let query = ListVideos {
		video_repository: &repository,
	};

	let resolved_limit = limit.unwrap_or(DEFAULT_PAGE_LIMIT);

	let page = query
		.by_newest(resolved_limit, cursor)
		.await
		.map_err(AppServerError::from)?;

	Ok(page.into())
}

#[server]
pub async fn get_trending_videos(
	limit: Option<u32>,
	cursor: Option<String>,
) -> Result<VideoCardPage, AppServerError> {
	let pool = use_context::<sqlx::PgPool>()
		.require_context("Missing pool")?;

	let repository = PgVideoRepository::new(&pool);
	let query = ListVideos {
		video_repository: &repository,
	};

	let resolved_limit = limit.unwrap_or(DEFAULT_PAGE_LIMIT);

	let page = query
		.by_most_popular(resolved_limit, cursor)
		.await
		.map_err(AppServerError::from)?;

	Ok(page.into())
}

#[server]
pub async fn get_videos_by_search(
	searched: String,
	limit: Option<u32>,
	cursor: Option<String>,
) -> Result<VideoCardPage, AppServerError> {
	let pool = use_context::<sqlx::PgPool>()
		.require_context("Missing pool")?;

	let repository = PgVideoRepository::new(&pool);
	let query = ListVideos {
		video_repository: &repository,
	};

	let resolved_limit = limit.unwrap_or(DEFAULT_PAGE_LIMIT);

	let page = query
		.by_title_regex(&searched, resolved_limit, cursor)
		.await
		.map_err(AppServerError::from)?;

	Ok(page.into())
}

#[server]
pub async fn get_channel_videos(
	user_id: String,
	limit: Option<u32>,
	cursor: Option<String>,
) -> Result<VideoCardPage, AppServerError> {
	let pool = use_context::<sqlx::PgPool>()
		.require_context("Missing pool")?;

	let repository = PgVideoRepository::new(&pool);
	let query = ListVideos {
		video_repository: &repository,
	};

	let resolved_limit = limit.unwrap_or(DEFAULT_PAGE_LIMIT);

	let page = query
		.by_user_id(user_id, resolved_limit, cursor)
		.await
		.map_err(AppServerError::from)?;

	Ok(page.into())
}
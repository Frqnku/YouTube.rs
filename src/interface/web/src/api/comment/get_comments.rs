use leptos::prelude::*;

#[cfg(feature = "ssr")]
use application::queries::{CountVideoComments, GetCommentById, ListCommentReplies, ListVideoComments};
#[cfg(feature = "ssr")]
use domain::_shared::DomainError;
#[cfg(feature = "ssr")]
use infrastructure::repositories::PgCommentRepository;

use crate::api::{_dtos::comment::{CommentDto, CommentPageDto}, _errors::AppServerError};
#[cfg(feature = "ssr")]
use crate::api::_errors::OptionExt;
#[cfg(feature = "ssr")]
use crate::context::CurrentUser;

#[server]
pub async fn get_video_comments(
	video_id: String,
	limit: Option<u32>,
	cursor: Option<String>,
	sort: Option<String>,
) -> Result<CommentPageDto, AppServerError> {
	let pool = use_context::<sqlx::PgPool>()
		.require_context("Missing pool")?;
	let viewer_user_id = use_context::<CurrentUser>().map(|user| user.id);

	let repository = PgCommentRepository::new(&pool);
	let query = ListVideoComments {
		comment_repository: &repository,
	};

	let page = query
		.execute(video_id, limit.unwrap_or_default(), cursor, sort, viewer_user_id)
		.await
		.map_err(AppServerError::from)?;

	Ok(page.into())
}

#[server]
pub async fn get_comment_replies(
	parent_comment_id: String,
	limit: Option<u32>,
	cursor: Option<String>,
	sort: Option<String>,
) -> Result<CommentPageDto, AppServerError> {
	let pool = use_context::<sqlx::PgPool>()
		.require_context("Missing pool")?;
	let viewer_user_id = use_context::<CurrentUser>().map(|user| user.id);

	let repository = PgCommentRepository::new(&pool);
	let query = ListCommentReplies {
		comment_repository: &repository,
	};

	let page = query
		.execute(parent_comment_id, limit.unwrap_or_default(), cursor, sort, viewer_user_id)
		.await
		.map_err(AppServerError::from)?;

	Ok(page.into())
}

#[server]
pub async fn get_comment(comment_id: String) -> Result<CommentDto, AppServerError> {
	let pool = use_context::<sqlx::PgPool>()
		.require_context("Missing pool")?;
	let viewer_user_id = use_context::<CurrentUser>().map(|user| user.id);

	let repository = PgCommentRepository::new(&pool);
	let query = GetCommentById {
		comment_repository: &repository,
	};

	let comment = query
		.execute(comment_id, viewer_user_id)
		.await
		.map_err(AppServerError::from)?
		.ok_or_else(|| AppServerError::from(DomainError::BadRequest("Comment not found".to_string())))?;

	Ok(comment.into())
}

#[server]
pub async fn get_video_comment_count(video_id: String) -> Result<i64, AppServerError> {
	let pool = use_context::<sqlx::PgPool>()
		.require_context("Missing pool")?;

	let repository = PgCommentRepository::new(&pool);
	let query = CountVideoComments {
		comment_repository: &repository,
	};

	query
		.execute(video_id)
		.await
		.map_err(AppServerError::from)
}

use leptos::prelude::*;

#[cfg(feature = "ssr")]
use application::commands::{CreateComment, DeleteComment as DeleteCommentUseCase, UpdateCommentContent};
#[cfg(feature = "ssr")]
use domain::_shared::DomainError;
#[cfg(feature = "ssr")]
use infrastructure::repositories::PgCommentRepository;

use crate::api::{_dtos::comment::CommentDto, _errors::AppServerError};
#[cfg(feature = "ssr")]
use crate::api::_errors::OptionExt;
#[cfg(feature = "ssr")]
use crate::app::CurrentUser;

#[server]
pub async fn post_comment(
	video_id: String,
	content: String,
	parent_id: Option<String>,
) -> Result<CommentDto, AppServerError> {
	let pool = use_context::<sqlx::PgPool>()
		.require_context("Missing pool")?;

	let current_user = use_context::<CurrentUser>()
		.ok_or_else(|| AppServerError::from(DomainError::Unauthorized))?;

	let repository = PgCommentRepository::new(&pool);
	let command = CreateComment {
		comment_repository: &repository,
	};

	let comment = command
		.execute(
			video_id,
			current_user.id,
			current_user.name,
			current_user.profile_picture,
			parent_id,
			content,
		)
		.await
		.map_err(AppServerError::from)?;

	Ok(comment.into())
}

#[server]
pub async fn patch_comment(comment_id: String, content: String) -> Result<CommentDto, AppServerError> {
	let pool = use_context::<sqlx::PgPool>()
		.require_context("Missing pool")?;

	let current_user = use_context::<CurrentUser>()
		.ok_or_else(|| AppServerError::from(DomainError::Unauthorized))?;

	let repository = PgCommentRepository::new(&pool);
	let command = UpdateCommentContent {
		comment_repository: &repository,
	};

	let comment = command
		.execute(comment_id, current_user.id, content)
		.await
		.map_err(AppServerError::from)?;

	Ok(comment.into())
}

#[server]
pub async fn delete_comment(comment_id: String) -> Result<(), AppServerError> {
	let pool = use_context::<sqlx::PgPool>()
		.require_context("Missing pool")?;

	let current_user = use_context::<CurrentUser>()
		.ok_or_else(|| AppServerError::from(DomainError::Unauthorized))?;

	let repository = PgCommentRepository::new(&pool);
	let command = DeleteCommentUseCase {
		comment_repository: &repository,
	};

	command
		.execute(comment_id, current_user.id)
		.await
		.map_err(AppServerError::from)?;

	Ok(())
}

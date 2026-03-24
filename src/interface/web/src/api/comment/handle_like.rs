use leptos::prelude::*;

#[cfg(feature = "ssr")]
use application::commands::{AddCommentLike, RemoveCommentLike};
#[cfg(feature = "ssr")]
use domain::_shared::DomainError;
#[cfg(feature = "ssr")]
use infrastructure::repositories::PgCommentRepository;

use crate::api::_errors::AppServerError;
#[cfg(feature = "ssr")]
use crate::api::_errors::OptionExt;
#[cfg(feature = "ssr")]
use crate::context::CurrentUser;

#[server]
pub async fn post_comment_like(comment_id: String) -> Result<(), AppServerError> {
	let pool = use_context::<sqlx::PgPool>()
		.require_context("Missing pool")?;

	let current_user = use_context::<CurrentUser>()
		.ok_or_else(|| AppServerError::from(DomainError::Unauthorized))?;

	let repository = PgCommentRepository::new(&pool);
	let command = AddCommentLike {
		comment_like_repository: &repository,
	};

	command
		.execute(comment_id, current_user.id)
		.await
		.map_err(AppServerError::from)?;

	Ok(())
}

#[server]
pub async fn delete_comment_like(comment_id: String) -> Result<(), AppServerError> {
	let pool = use_context::<sqlx::PgPool>()
		.require_context("Missing pool")?;

	let current_user = use_context::<CurrentUser>()
		.ok_or_else(|| AppServerError::from(DomainError::Unauthorized))?;

	let repository = PgCommentRepository::new(&pool);
	let command = RemoveCommentLike {
		comment_like_repository: &repository,
	};

	command
		.execute(comment_id, current_user.id)
		.await
		.map_err(AppServerError::from)?;

	Ok(())
}

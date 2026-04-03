use leptos::prelude::*;

#[cfg(feature = "ssr")]
use application::commands::{CleanHistory as CleanHistoryUseCase};
#[cfg(feature = "ssr")]
use domain::_shared::DomainError;
#[cfg(feature = "ssr")]
use infrastructure::repositories::PgVideoRepository;

use crate::api::_errors::AppServerError;
#[cfg(feature = "ssr")]
use crate::api::_errors::OptionExt;
#[cfg(feature = "ssr")]
use crate::context::CurrentUser;

#[server]
pub async fn clean_history() -> Result<(), AppServerError> {
	let pool = use_context::<sqlx::PgPool>()
		.require_context("Missing pool")?;

	let current_user = use_context::<CurrentUser>()
		.ok_or_else(|| AppServerError::from(DomainError::Unauthorized))?;

	let repository = PgVideoRepository::new(&pool);
	let command = CleanHistoryUseCase {
		video_repository: &repository,
	};

	command
		.execute(current_user.id)
		.await
		.map_err(AppServerError::from)?;

	Ok(())
}
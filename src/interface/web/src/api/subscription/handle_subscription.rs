use leptos::prelude::*;

#[cfg(feature = "ssr")]
use application::commands::{Subscribe, Unsubscribe};
#[cfg(feature = "ssr")]
use domain::_shared::DomainError;
#[cfg(feature = "ssr")]
use infrastructure::repositories::PgSubscriptionRepository;

use crate::api::_errors::AppServerError;
#[cfg(feature = "ssr")]
use crate::api::_errors::OptionExt;
#[cfg(feature = "ssr")]
use crate::app::CurrentUser;

#[server]
pub async fn post_subscription(channel_id: String) -> Result<(), AppServerError> {
	let pool = use_context::<sqlx::PgPool>()
		.require_context("Missing pool")?;

	let current_user = use_context::<CurrentUser>()
		.ok_or_else(|| AppServerError::from(DomainError::Unauthorized))?;

	let repository = PgSubscriptionRepository::new(&pool);
	let command = Subscribe {
		subscription_repository: &repository,
	};

	command
		.execute(current_user.id, channel_id)
		.await
		.map_err(AppServerError::from)?;

	Ok(())
}

#[server]
pub async fn delete_subscription(channel_id: String) -> Result<(), AppServerError> {
	let pool = use_context::<sqlx::PgPool>()
		.require_context("Missing pool")?;

	let current_user = use_context::<CurrentUser>()
		.ok_or_else(|| AppServerError::from(DomainError::Unauthorized))?;

	let repository = PgSubscriptionRepository::new(&pool);
	let command = Unsubscribe {
		subscription_repository: &repository,
	};

	command
		.execute(current_user.id, channel_id)
		.await
		.map_err(AppServerError::from)?;

	Ok(())
}

use leptos::prelude::*;

#[cfg(feature = "ssr")]
use application::queries::{CountSubscribers, GetSubscriptionStatus as GetSubscriptionStatusUsecase, ListSubscriptions};
#[cfg(feature = "ssr")]
use domain::_shared::DomainError;
#[cfg(feature = "ssr")]
use infrastructure::repositories::PgSubscriptionRepository;

use crate::api::{_dtos::subscription::ChannelDto, _errors::AppServerError};
#[cfg(feature = "ssr")]
use crate::api::_errors::OptionExt;
#[cfg(feature = "ssr")]
use crate::app::CurrentUser;

#[server]
pub async fn get_subscription_status(channel_id: String) -> Result<bool, AppServerError> {
	let pool = use_context::<sqlx::PgPool>()
		.require_context("Missing pool")?;

	let current_user = use_context::<CurrentUser>()
		.ok_or_else(|| AppServerError::from(DomainError::Unauthorized))?;

	let repository = PgSubscriptionRepository::new(&pool);
	let query = GetSubscriptionStatusUsecase {
		subscription_repository: &repository,
	};

	query
		.execute(current_user.id, channel_id)
		.await
		.map_err(AppServerError::from)
}

#[server]
pub async fn get_subscriptions() -> Result<Vec<ChannelDto>, AppServerError> {
	let pool = use_context::<sqlx::PgPool>()
		.require_context("Missing pool")?;

	let current_user = use_context::<CurrentUser>()
		.ok_or_else(|| AppServerError::from(DomainError::Unauthorized))?;

	let repository = PgSubscriptionRepository::new(&pool);
	let query = ListSubscriptions {
		subscription_repository: &repository,
	};

	let channels = query
		.execute(current_user.id)
		.await
		.map_err(AppServerError::from)?;

	Ok(channels.into_iter().map(ChannelDto::from).collect())
}

#[server]
pub async fn get_subscriber_count(channel_id: String) -> Result<usize, AppServerError> {
	let pool = use_context::<sqlx::PgPool>()
		.require_context("Missing pool")?;

	let repository = PgSubscriptionRepository::new(&pool);
	let query = CountSubscribers {
		subscription_repository: &repository,
	};

	query
		.execute(channel_id)
		.await
		.map_err(AppServerError::from)
}

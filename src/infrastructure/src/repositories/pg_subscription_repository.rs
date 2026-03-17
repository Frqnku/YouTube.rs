use anyhow::Context;
use domain::{
	_shared::value_objects::Url,
	subscription::SubscriptionRepository,
	user::{entity::User, value_objects::Email},
};
use sqlx::types::Uuid;

pub struct PgSubscriptionRepository {
	pool: sqlx::PgPool,
}

impl PgSubscriptionRepository {
	pub fn new(pool: &sqlx::PgPool) -> Self {
		Self { pool: pool.clone() }
	}
}

#[derive(sqlx::FromRow)]
struct ChannelRecord {
	id: Uuid,
	name: String,
	email: String,
	profile_picture: Option<String>,
}

impl ChannelRecord {
	fn into_user(self) -> anyhow::Result<User> {
		let profile_picture = self
			.profile_picture
			.map(Url::try_from)
			.transpose()?;

		Ok(User::new(
			self.id,
			self.name,
			Email::try_from(self.email)?,
			profile_picture,
		))
	}
}

#[async_trait::async_trait]
impl SubscriptionRepository for PgSubscriptionRepository {
	async fn subscribe(&self, subscriber_id: Uuid, channel_id: Uuid) -> anyhow::Result<()> {
		sqlx::query(
			"INSERT INTO subscriptions (subscriber_id, channel_id)
			 VALUES ($1, $2)
			 ON CONFLICT DO NOTHING",
		)
		.bind(subscriber_id)
		.bind(channel_id)
		.execute(&self.pool)
		.await?;

		Ok(())
	}

	async fn unsubscribe(&self, subscriber_id: Uuid, channel_id: Uuid) -> anyhow::Result<()> {
		sqlx::query(
			"DELETE FROM subscriptions
			 WHERE subscriber_id = $1 AND channel_id = $2",
		)
		.bind(subscriber_id)
		.bind(channel_id)
		.execute(&self.pool)
		.await?;

		Ok(())
	}

	async fn is_subscribed(&self, subscriber_id: Uuid, channel_id: Uuid) -> anyhow::Result<bool> {
		let is_subscribed = sqlx::query_scalar::<_, bool>(
			"SELECT EXISTS(
				SELECT 1
				FROM subscriptions
				WHERE subscriber_id = $1 AND channel_id = $2
			)",
		)
		.bind(subscriber_id)
		.bind(channel_id)
		.fetch_one(&self.pool)
		.await?;

		Ok(is_subscribed)
	}

	async fn list_subscriptions(&self, subscriber_id: Uuid) -> anyhow::Result<Vec<User>> {
		let records = sqlx::query_as::<_, ChannelRecord>(
			"SELECT u.id, u.name, u.email, u.profile_picture
			 FROM subscriptions s
			 JOIN users u ON u.id = s.channel_id
			 WHERE s.subscriber_id = $1
			 ORDER BY s.created_at DESC",
		)
		.bind(subscriber_id)
		.fetch_all(&self.pool)
		.await?;

		records
			.into_iter()
			.map(ChannelRecord::into_user)
			.collect()
	}

	async fn count_subscribers(&self, channel_id: Uuid) -> anyhow::Result<usize> {
		let total = sqlx::query_scalar::<_, i64>(
			"SELECT COUNT(*)::bigint
			 FROM subscriptions
			 WHERE channel_id = $1",
		)
		.bind(channel_id)
		.fetch_one(&self.pool)
		.await?;

		usize::try_from(total).context("Subscriber count overflow")
	}
}

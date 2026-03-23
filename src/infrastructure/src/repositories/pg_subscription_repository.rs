use anyhow::Context;
use domain::{
	_shared::value_objects::Url,
	channel::{entity::Channel, SubscriptionRepository},
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
	profile_picture: Option<String>,
	banner_url: Option<String>,
	description: Option<String>,
	subscriber_count: i64,
	video_count: i64,
}

impl ChannelRecord {
	fn into_channel(self) -> anyhow::Result<Channel> {
		let profile_picture = self
			.profile_picture
			.map(Url::try_from)
			.transpose()?;
		let banner = self.banner_url.map(Url::try_from).transpose()?;
		let subscriber_count = usize::try_from(self.subscriber_count)
			.context("Subscriber count overflow")?;
		let video_count = usize::try_from(self.video_count)
			.context("Video count overflow")?;

		Ok(Channel::new(
			self.id,
			self.name,
			profile_picture,
			banner,
			self.description,
			subscriber_count,
			video_count,
		))
	}
}

#[async_trait::async_trait]
impl SubscriptionRepository for PgSubscriptionRepository {
	async fn subscribe(&self, subscriber_id: Uuid, channel_id: Uuid) -> anyhow::Result<()> {
		let mut tx = self.pool.begin().await?;

		sqlx::query(
			"INSERT INTO channels (user_id)
			 VALUES ($1)
			 ON CONFLICT (user_id) DO NOTHING",
		)
		.bind(channel_id)
		.execute(&mut *tx)
		.await?;

		let insert_result = sqlx::query(
			"INSERT INTO subscriptions (subscriber_id, channel_id)
			 VALUES ($1, $2)
			 ON CONFLICT DO NOTHING",
		)
		.bind(subscriber_id)
		.bind(channel_id)
		.execute(&mut *tx)
		.await?;

		if insert_result.rows_affected() > 0 {
			sqlx::query(
				"UPDATE channels
				 SET subscriber_count = subscriber_count + 1
				 WHERE user_id = $1",
			)
			.bind(channel_id)
			.execute(&mut *tx)
			.await?;
		}

		tx.commit().await?;

		Ok(())
	}

	async fn unsubscribe(&self, subscriber_id: Uuid, channel_id: Uuid) -> anyhow::Result<()> {
		let mut tx = self.pool.begin().await?;

		let delete_result = sqlx::query(
			"DELETE FROM subscriptions
			 WHERE subscriber_id = $1 AND channel_id = $2",
		)
		.bind(subscriber_id)
		.bind(channel_id)
		.execute(&mut *tx)
		.await?;

		if delete_result.rows_affected() > 0 {
			sqlx::query(
				"UPDATE channels
				 SET subscriber_count = GREATEST(subscriber_count - 1, 0)
				 WHERE user_id = $1",
			)
			.bind(channel_id)
			.execute(&mut *tx)
			.await?;
		}

		tx.commit().await?;

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

	async fn list_subscriptions(&self, subscriber_id: Uuid) -> anyhow::Result<Vec<Channel>> {
		let records = sqlx::query_as!(
			ChannelRecord,
			"SELECT u.id,
			        u.name,
			        u.profile_picture,
			        c.banner_url,
			        c.description,
			        COALESCE(c.subscriber_count, 0) AS \"subscriber_count!\",
			        COALESCE(c.video_count, 0) AS \"video_count!\"
			 FROM subscriptions s
			 JOIN users u ON u.id = s.channel_id
			 LEFT JOIN channels c ON c.user_id = u.id
			 WHERE s.subscriber_id = $1
			 ORDER BY s.created_at DESC",
			subscriber_id,
		)
		.fetch_all(&self.pool)
		.await?;

		records
			.into_iter()
			.map(ChannelRecord::into_channel)
			.collect()
	}

	async fn count_subscribers(&self, channel_id: Uuid) -> anyhow::Result<usize> {
		let total = sqlx::query_scalar::<_, i64>(
			"SELECT subscriber_count
			 FROM channels
			 WHERE user_id = $1",
		)
		.bind(channel_id)
		.fetch_one(&self.pool)
		.await?;

		usize::try_from(total).context("Subscriber count overflow")
	}
}

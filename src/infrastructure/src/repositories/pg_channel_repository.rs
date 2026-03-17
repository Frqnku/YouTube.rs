use anyhow::Context;
use domain::{
	_shared::value_objects::Url,
	channel::{entity::Channel, ChannelRepository},
};
use sqlx::types::Uuid;

pub struct PgChannelRepository {
	pool: sqlx::PgPool,
}

impl PgChannelRepository {
	pub fn new(pool: &sqlx::PgPool) -> Self {
		Self { pool: pool.clone() }
	}
}

#[derive(sqlx::FromRow)]
struct ChannelRecord {
	id: Uuid,
	name: String,
	profile_picture: Option<String>,
	description: Option<String>,
	subscriber_count: i64,
	video_count: i64,
}

impl ChannelRecord {
	fn into_channel(self) -> anyhow::Result<Channel> {
		let profile_picture = self.profile_picture.map(Url::try_from).transpose()?;
		let subscriber_count = usize::try_from(self.subscriber_count)
			.context("Subscriber count overflow")?;
		let video_count =
			usize::try_from(self.video_count).context("Video count overflow")?;

		Ok(Channel::new(
			self.id,
			self.name,
			profile_picture,
			self.description,
			subscriber_count,
			video_count,
		))
	}
}

#[async_trait::async_trait]
impl ChannelRepository for PgChannelRepository {
	async fn find_channel_by_id(&self, channel_id: Uuid) -> anyhow::Result<Option<Channel>> {
		let record = sqlx::query_as::<_, ChannelRecord>(
			"SELECT u.id,
					u.name,
					u.profile_picture,
					c.description,
					c.subscriber_count,
					(
						SELECT COUNT(*)::bigint
						FROM videos v
						WHERE v.user_id = u.id
					) AS video_count
			 FROM users u
			 LEFT JOIN channels c ON c.user_id = u.id
			 WHERE u.id = $1",
		)
		.bind(channel_id)
		.fetch_optional(&self.pool)
		.await?;

		record
			.map(ChannelRecord::into_channel)
			.transpose()
	}
}

use anyhow::{Context, anyhow};
use domain::_shared::value_objects::Url;
use domain::comment::entity::{Comment, CommentAuthor};
use domain::comment::{
	CommentLikeRepository,
	CommentPage,
	CommentPageRequest,
	CommentRepository,
	CommentSort,
};
use sqlx::types::Uuid;
use sqlx::types::time::OffsetDateTime;

pub struct PgCommentRepository {
	pool: sqlx::PgPool,
}

impl PgCommentRepository {
	pub fn new(pool: &sqlx::PgPool) -> Self {
		Self { pool: pool.clone() }
	}
}

#[derive(sqlx::FromRow)]
struct CommentRecord {
	id: Uuid,
	video_id: Uuid,
	user_id: Uuid,
	username: String,
	user_picture: Option<String>,
	parent_id: Option<Uuid>,
	content: String,
	like_count: i64,
	reply_count: i64,
	liked_by_viewer: Option<bool>,
	created_at: OffsetDateTime,
	updated_at: OffsetDateTime,
}

impl CommentRecord {
	fn into_comment(self) -> anyhow::Result<Comment> {
		let created_at = chrono::DateTime::<chrono::Utc>::from_timestamp(
			self.created_at.unix_timestamp(),
			self.created_at.nanosecond(),
		)
		.ok_or_else(|| anyhow!("Invalid created_at timestamp from database"))?;

		let updated_at = chrono::DateTime::<chrono::Utc>::from_timestamp(
			self.updated_at.unix_timestamp(),
			self.updated_at.nanosecond(),
		)
		.ok_or_else(|| anyhow!("Invalid updated_at timestamp from database"))?;

		let mut comment = Comment::new(
			self.id,
			self.video_id,
			CommentAuthor::new(
				self.user_id,
				self.username,
				self.user_picture.map(Url::try_from).transpose()?,
			),
			self.parent_id,
			self.content,
			self.like_count,
			self.reply_count,
			self.liked_by_viewer,
		);

		comment.created_at = created_at;
		comment.updated_at = updated_at;

		Ok(comment)
	}
}

const COMMENT_SELECT_WITH_USER: &str = "SELECT
	c.id,
	c.video_id,
	c.user_id,
	u.name AS username,
	u.profile_picture AS user_picture,
	c.parent_id,
	c.content,
	c.like_count::bigint AS like_count,
	(
		SELECT COUNT(*)::bigint
		FROM video_comments child
		WHERE child.parent_id = c.id
	) AS reply_count,
	CASE
		WHEN $1::uuid IS NULL THEN NULL
		ELSE (viewer_cl.user_id IS NOT NULL)
	END AS liked_by_viewer,
	c.created_at,
	c.updated_at
 FROM video_comments c
 JOIN users u ON u.id = c.user_id
 LEFT JOIN comment_likes viewer_cl ON viewer_cl.comment_id = c.id AND viewer_cl.user_id = $1";

fn comment_query_sql(suffix: &str) -> String {
	format!("{COMMENT_SELECT_WITH_USER} {suffix}")
}

fn limit_plus_one(page: &CommentPageRequest) -> i64 {
	i64::from(page.limit.saturating_add(1))
}

fn newest_or_oldest_cursor(record: &CommentRecord) -> String {
	format!("{}|{}", record.created_at.unix_timestamp_nanos(), record.id)
}

fn popular_cursor(record: &CommentRecord) -> String {
	format!("{}|{}", record.like_count, record.id)
}

fn parse_created_cursor(cursor: &str) -> anyhow::Result<(OffsetDateTime, Uuid)> {
	let (ts, id) = cursor
		.split_once('|')
		.context("Invalid comment cursor format")?;

	let nanos: i128 = ts.parse().context("Invalid comment cursor timestamp")?;
	let at = OffsetDateTime::from_unix_timestamp_nanos(nanos)
		.map_err(|_| anyhow!("Invalid comment cursor timestamp range"))?;
	let id = Uuid::parse_str(id).context("Invalid comment cursor id")?;

	Ok((at, id))
}

fn parse_popular_cursor(cursor: &str) -> anyhow::Result<(i64, Uuid)> {
	let (likes, id) = cursor
		.split_once('|')
		.context("Invalid comment popular cursor format")?;

	let like_count: i64 = likes.parse().context("Invalid comment popular like_count")?;
	let id = Uuid::parse_str(id).context("Invalid comment popular cursor id")?;

	Ok((like_count, id))
}

fn into_page(mut records: Vec<CommentRecord>, page: &CommentPageRequest) -> anyhow::Result<CommentPage> {
	let has_more = records.len() > page.limit as usize;
	if has_more {
		records.truncate(page.limit as usize);
	}

	let next_cursor = if has_more {
		records.last().map(|record| match page.sort {
			CommentSort::Newest | CommentSort::Oldest => newest_or_oldest_cursor(record),
			CommentSort::MostLiked => popular_cursor(record),
		})
	} else {
		None
	};

	let items = records
		.into_iter()
		.map(CommentRecord::into_comment)
		.collect::<anyhow::Result<Vec<_>>>()?;

	Ok(CommentPage::new(items, next_cursor, has_more))
}

#[async_trait::async_trait]
impl CommentRepository for PgCommentRepository {
	async fn find_by_id(&self, id: Uuid, viewer_user_id: Option<Uuid>) -> anyhow::Result<Option<Comment>> {
		let sql = comment_query_sql("WHERE c.id = $2");
		let record = sqlx::query_as::<_, CommentRecord>(&sql)
			.bind(viewer_user_id)
			.bind(id)
			.fetch_optional(&self.pool)
			.await?;

		record
			.map(CommentRecord::into_comment)
			.transpose()
	}

	async fn list_by_video_id(
		&self,
		video_id: Uuid,
		parent_id: Option<Uuid>,
		page: CommentPageRequest,
		viewer_user_id: Option<Uuid>,
	) -> anyhow::Result<CommentPage> {
		let records = match page.sort {
			CommentSort::Newest => {
				if let Some(cursor) = page.cursor.as_deref() {
					let (created_at, id) = parse_created_cursor(cursor)?;
					let sql = comment_query_sql(
						"WHERE c.video_id = $2
							AND c.parent_id IS NOT DISTINCT FROM $3
							AND (c.created_at, c.id) < ($4, $5)
						 ORDER BY c.created_at DESC, c.id DESC
						 LIMIT $6",
					);

					sqlx::query_as::<_, CommentRecord>(&sql)
						.bind(viewer_user_id)
						.bind(video_id)
						.bind(parent_id)
						.bind(created_at)
						.bind(id)
						.bind(limit_plus_one(&page))
						.fetch_all(&self.pool)
						.await?
				} else {
					let sql = comment_query_sql(
						"WHERE c.video_id = $2
							AND c.parent_id IS NOT DISTINCT FROM $3
						 ORDER BY c.created_at DESC, c.id DESC
						 LIMIT $4",
					);

					sqlx::query_as::<_, CommentRecord>(&sql)
						.bind(viewer_user_id)
						.bind(video_id)
						.bind(parent_id)
						.bind(limit_plus_one(&page))
						.fetch_all(&self.pool)
						.await?
				}
			}
			CommentSort::Oldest => {
				if let Some(cursor) = page.cursor.as_deref() {
					let (created_at, id) = parse_created_cursor(cursor)?;
					let sql = comment_query_sql(
						"WHERE c.video_id = $2
							AND c.parent_id IS NOT DISTINCT FROM $3
							AND (c.created_at, c.id) > ($4, $5)
						 ORDER BY c.created_at ASC, c.id ASC
						 LIMIT $6",
					);

					sqlx::query_as::<_, CommentRecord>(&sql)
						.bind(viewer_user_id)
						.bind(video_id)
						.bind(parent_id)
						.bind(created_at)
						.bind(id)
						.bind(limit_plus_one(&page))
						.fetch_all(&self.pool)
						.await?
				} else {
					let sql = comment_query_sql(
						"WHERE c.video_id = $2
							AND c.parent_id IS NOT DISTINCT FROM $3
						 ORDER BY c.created_at ASC, c.id ASC
						 LIMIT $4",
					);

					sqlx::query_as::<_, CommentRecord>(&sql)
						.bind(viewer_user_id)
						.bind(video_id)
						.bind(parent_id)
						.bind(limit_plus_one(&page))
						.fetch_all(&self.pool)
						.await?
				}
			}
			CommentSort::MostLiked => {
				if let Some(cursor) = page.cursor.as_deref() {
					let (like_count, id) = parse_popular_cursor(cursor)?;
					let sql = comment_query_sql(
						"WHERE c.video_id = $2
							AND c.parent_id IS NOT DISTINCT FROM $3
							AND (c.like_count, c.id) < ($4, $5)
						 ORDER BY c.like_count DESC, c.id DESC
						 LIMIT $6",
					);

					sqlx::query_as::<_, CommentRecord>(&sql)
						.bind(viewer_user_id)
						.bind(video_id)
						.bind(parent_id)
						.bind(like_count)
						.bind(id)
						.bind(limit_plus_one(&page))
						.fetch_all(&self.pool)
						.await?
				} else {
					let sql = comment_query_sql(
						"WHERE c.video_id = $2
							AND c.parent_id IS NOT DISTINCT FROM $3
						 ORDER BY c.like_count DESC, c.id DESC
						 LIMIT $4",
					);

					sqlx::query_as::<_, CommentRecord>(&sql)
						.bind(viewer_user_id)
						.bind(video_id)
						.bind(parent_id)
						.bind(limit_plus_one(&page))
						.fetch_all(&self.pool)
						.await?
				}
			}
		};

		into_page(records, &page)
	}

	async fn list_replies(
		&self,
		parent_id: Uuid,
		page: CommentPageRequest,
		viewer_user_id: Option<Uuid>,
	) -> anyhow::Result<CommentPage> {
		let parent = self.find_by_id(parent_id, viewer_user_id).await?;
		let parent = parent.ok_or_else(|| anyhow!("Parent comment not found"))?;

		self.list_by_video_id(parent.video_id, Some(parent_id), page, viewer_user_id).await
	}

	async fn count_by_video_id(&self, video_id: Uuid) -> anyhow::Result<i64> {
		let count = sqlx::query_scalar::<_, i64>(
			"SELECT COUNT(*)::bigint
			 FROM video_comments
			 WHERE video_id = $1",
		)
		.bind(video_id)
		.fetch_one(&self.pool)
		.await?;

		Ok(count)
	}

	async fn count_replies(&self, parent_id: Uuid) -> anyhow::Result<i64> {
		let count = sqlx::query_scalar::<_, i64>(
			"SELECT COUNT(*)::bigint
			 FROM video_comments
			 WHERE parent_id = $1",
		)
		.bind(parent_id)
		.fetch_one(&self.pool)
		.await?;

		Ok(count)
	}

	async fn save(&self, comment: &Comment) -> anyhow::Result<Comment> {
		sqlx::query(
			"INSERT INTO video_comments (id, video_id, user_id, parent_id, content, like_count)
			 VALUES ($1, $2, $3, $4, $5, $6)",
		)
		.bind(comment.id)
		.bind(comment.video_id)
		.bind(comment.author.id)
		.bind(comment.parent_id)
		.bind(&comment.content)
		.bind(comment.like_count as i32)
		.execute(&self.pool)
		.await?;

		self.find_by_id(comment.id, None)
			.await?
			.ok_or_else(|| anyhow!("Inserted comment not found"))
	}

	async fn update_content(&self, comment_id: Uuid, user_id: Uuid, content: String) -> anyhow::Result<Comment> {
		let result = sqlx::query(
			"UPDATE video_comments
			 SET content = $3, updated_at = NOW()
			 WHERE id = $1 AND user_id = $2",
		)
		.bind(comment_id)
		.bind(user_id)
		.bind(content)
		.execute(&self.pool)
		.await?;

		if result.rows_affected() == 0 {
			return Err(anyhow!("Comment not found or unauthorized"));
		}

		self.find_by_id(comment_id, Some(user_id))
			.await?
			.ok_or_else(|| anyhow!("Updated comment not found"))
	}

	async fn delete(&self, comment_id: Uuid, user_id: Uuid) -> anyhow::Result<()> {
		let result = sqlx::query(
			"DELETE FROM video_comments
			 WHERE id = $1 AND user_id = $2",
		)
		.bind(comment_id)
		.bind(user_id)
		.execute(&self.pool)
		.await?;

		if result.rows_affected() == 0 {
			return Err(anyhow!("Comment not found or unauthorized"));
		}

		Ok(())
	}
}

#[async_trait::async_trait]
impl CommentLikeRepository for PgCommentRepository {
	async fn is_liked_by_user(&self, comment_id: Uuid, user_id: Uuid) -> anyhow::Result<bool> {
		let liked = sqlx::query_scalar::<_, bool>(
			"SELECT EXISTS(
				SELECT 1
				FROM comment_likes
				WHERE comment_id = $1 AND user_id = $2
			)",
		)
		.bind(comment_id)
		.bind(user_id)
		.fetch_one(&self.pool)
		.await?;

		Ok(liked)
	}

	async fn add_like(&self, comment_id: Uuid, user_id: Uuid) -> anyhow::Result<()> {
		let mut tx = self.pool.begin().await?;

		let insert_result = sqlx::query(
			"INSERT INTO comment_likes (comment_id, user_id)
			 VALUES ($1, $2)
			 ON CONFLICT DO NOTHING",
		)
		.bind(comment_id)
		.bind(user_id)
		.execute(&mut *tx)
		.await?;

		if insert_result.rows_affected() > 0 {
			sqlx::query(
				"UPDATE video_comments
				 SET like_count = like_count + 1, updated_at = NOW()
				 WHERE id = $1",
			)
			.bind(comment_id)
			.execute(&mut *tx)
			.await?;
		}

		tx.commit().await?;
		Ok(())
	}

	async fn remove_like(&self, comment_id: Uuid, user_id: Uuid) -> anyhow::Result<()> {
		let mut tx = self.pool.begin().await?;

		let delete_result = sqlx::query(
			"DELETE FROM comment_likes
			 WHERE comment_id = $1 AND user_id = $2",
		)
		.bind(comment_id)
		.bind(user_id)
		.execute(&mut *tx)
		.await?;

		if delete_result.rows_affected() > 0 {
			sqlx::query(
				"UPDATE video_comments
				 SET like_count = GREATEST(like_count - 1, 0), updated_at = NOW()
				 WHERE id = $1",
			)
			.bind(comment_id)
			.execute(&mut *tx)
			.await?;
		}

		tx.commit().await?;
		Ok(())
	}
}

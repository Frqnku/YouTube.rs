use anyhow::{Context, anyhow};
use domain::_shared::value_objects::Url;
use domain::video::entity::{Video, VideoAuthor};
use domain::video::{PageRequest, VideoPage, VideoReactionRepository, VideoRepository, VideoViewRepository};
use sqlx::types::Uuid;
use sqlx::types::time::OffsetDateTime;

pub struct PgVideoRepository {
    pool: sqlx::PgPool,
}

impl PgVideoRepository {
    pub fn new(pool: &sqlx::PgPool) -> Self {
        Self { pool: pool.clone() }
    }
}

#[derive(sqlx::FromRow)]
struct VideoRecord {
    id: Uuid,
    user_id: Uuid,
    username: String,
    user_picture: Option<String>,
    title: String,
    description: String,
    video_url: String,
    thumbnail_url: String,
    preview_url: String,
    duration_seconds: i32,
    view_count: i64,
    like_count: i64,
    dislike_count: i64,
    created_at: OffsetDateTime,
}

impl VideoRecord {
    fn into_video(self) -> anyhow::Result<Video> {
        let created_at = chrono::DateTime::<chrono::Utc>::from_timestamp(
            self.created_at.unix_timestamp(),
            self.created_at.nanosecond(),
        )
        .ok_or_else(|| anyhow!("Invalid created_at timestamp from database"))?;

        let mut video = Video::new(
            self.id,
            VideoAuthor::new(
                self.user_id,
                self.username,
                self.user_picture.map(Url::try_from).transpose()?,
            ),
            self.title,
            self.description,
            Url::try_from(self.video_url.clone())?,
            Url::try_from(self.thumbnail_url)?,
            Url::try_from(self.preview_url)?, // Using video_url as preview_url for now
            self.duration_seconds,
            self.view_count,
            self.like_count,
            self.dislike_count,
        );

        video.created_at = created_at;

        Ok(video)
    }
}

const VIDEO_SELECT_WITH_USER: &str = "SELECT
    v.id,
    v.user_id,
    u.name AS username,
    u.profile_picture AS user_picture,
    v.title,
    v.description,
    v.video_url,
    v.thumbnail_url,
    v.preview_url,
    v.duration_seconds,
    v.view_count,
    v.like_count,
    v.dislike_count,
    v.created_at
 FROM videos v
 JOIN users u ON u.id = v.user_id";

fn video_query_sql(suffix: &str) -> String {
    format!("{VIDEO_SELECT_WITH_USER} {suffix}")
}

fn limit_plus_one(page: &PageRequest) -> i64 {
    i64::from(page.limit.saturating_add(1))
}

fn newest_cursor(record: &VideoRecord) -> String {
    format!("{}|{}", record.created_at.unix_timestamp_nanos(), record.id)
}

fn parse_newest_cursor(cursor: &str) -> anyhow::Result<(OffsetDateTime, Uuid)> {
    let (ts, id) = cursor
        .split_once('|')
        .context("Invalid newest cursor format")?;

    let nanos: i128 = ts.parse().context("Invalid newest cursor timestamp")?;
    let at = OffsetDateTime::from_unix_timestamp_nanos(nanos)
        .map_err(|_| anyhow!("Invalid newest cursor timestamp range"))?;
    let id = Uuid::parse_str(id).context("Invalid newest cursor id")?;

    Ok((at, id))
}

fn popular_cursor(record: &VideoRecord) -> String {
    format!("{}|{}", record.view_count, record.id)
}

fn parse_popular_cursor(cursor: &str) -> anyhow::Result<(i64, Uuid)> {
    let (count, id) = cursor
        .split_once('|')
        .context("Invalid popular cursor format")?;

    let count: i64 = count.parse().context("Invalid popular cursor view count")?;
    let id = Uuid::parse_str(id).context("Invalid popular cursor id")?;

    Ok((count, id))
}

fn into_page<F>(mut records: Vec<VideoRecord>, page: &PageRequest, cursor_fn: F) -> anyhow::Result<VideoPage>
where
    F: Fn(&VideoRecord) -> String,
{
    let has_more = records.len() > page.limit as usize;
    if has_more {
        records.truncate(page.limit as usize);
    }

    let next_cursor = if has_more {
        records.last().map(cursor_fn)
    } else {
        None
    };

    let items = records
        .into_iter()
        .map(VideoRecord::into_video)
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok(VideoPage::new(
        items,
        next_cursor,
        has_more,
    ))
}

#[async_trait::async_trait]
impl VideoRepository for PgVideoRepository {
    async fn find_by_id(&self, id: Uuid) -> Option<Video> {
        let sql = video_query_sql("WHERE v.id = $1");
        let record = sqlx::query_as::<_, VideoRecord>(&sql)
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .ok()?;

        record.and_then(|rec| rec.into_video().ok())
    }

    async fn list_newest(&self, page: PageRequest) -> anyhow::Result<VideoPage> {
        let records = if let Some(cursor) = page.cursor.as_deref() {
            let (created_at, id) = parse_newest_cursor(cursor)?;
            let sql = video_query_sql(
                "WHERE (v.created_at, v.id) < ($1, $2)
                 ORDER BY v.created_at DESC, v.id DESC
                 LIMIT $3",
            );
            sqlx::query_as::<_, VideoRecord>(&sql)
            .bind(created_at)
            .bind(id)
            .bind(limit_plus_one(&page))
            .fetch_all(&self.pool)
            .await?
        } else {
            let sql = video_query_sql("ORDER BY v.created_at DESC, v.id DESC LIMIT $1");
            sqlx::query_as::<_, VideoRecord>(&sql)
            .bind(limit_plus_one(&page))
            .fetch_all(&self.pool)
            .await?
        };

        into_page(records, &page, newest_cursor)
    }

    async fn list_most_popular(&self, page: PageRequest) -> anyhow::Result<VideoPage> {
        let records = if let Some(cursor) = page.cursor.as_deref() {
            let (view_count, id) = parse_popular_cursor(cursor)?;
            let sql = video_query_sql(
                "WHERE (v.view_count, v.id) < ($1, $2)
                 ORDER BY v.view_count DESC, v.id DESC
                 LIMIT $3",
            );
            sqlx::query_as::<_, VideoRecord>(&sql)
            .bind(view_count)
            .bind(id)
            .bind(limit_plus_one(&page))
            .fetch_all(&self.pool)
            .await?
        } else {
            let sql = video_query_sql("ORDER BY v.view_count DESC, v.id DESC LIMIT $1");
            sqlx::query_as::<_, VideoRecord>(&sql)
            .bind(limit_plus_one(&page))
            .fetch_all(&self.pool)
            .await?
        };

        into_page(records, &page, popular_cursor)
    }

    async fn list_by_user_id(&self, user_id: Uuid, page: PageRequest) -> anyhow::Result<VideoPage> {
        let records = if let Some(cursor) = page.cursor.as_deref() {
            let (created_at, id) = parse_newest_cursor(cursor)?;
            let sql = video_query_sql(
                "WHERE v.user_id = $1 AND (v.created_at, v.id) < ($2, $3)
                 ORDER BY v.created_at DESC, v.id DESC
                 LIMIT $4",
            );
            sqlx::query_as::<_, VideoRecord>(&sql)
            .bind(user_id)
            .bind(created_at)
            .bind(id)
            .bind(limit_plus_one(&page))
            .fetch_all(&self.pool)
            .await?
        } else {
            let sql = video_query_sql(
                "WHERE v.user_id = $1
                 ORDER BY v.created_at DESC, v.id DESC
                 LIMIT $2",
            );
            sqlx::query_as::<_, VideoRecord>(&sql)
            .bind(user_id)
            .bind(limit_plus_one(&page))
            .fetch_all(&self.pool)
            .await?
        };

        into_page(records, &page, newest_cursor)
    }

    async fn search_by_title(&self, query: &str, page: PageRequest) -> anyhow::Result<VideoPage> {
        let like_query = format!("%{}%", query);

        let records = if let Some(cursor) = page.cursor.as_deref() {
            let (created_at, id) = parse_newest_cursor(cursor)?;
            let sql = video_query_sql(
                "WHERE (v.title ILIKE $1 OR u.name ILIKE $1) AND (v.created_at, v.id) < ($2, $3)
                 ORDER BY v.created_at DESC, v.id DESC
                 LIMIT $4",
            );
            sqlx::query_as::<_, VideoRecord>(&sql)
            .bind(&like_query)
            .bind(created_at)
            .bind(id)
            .bind(limit_plus_one(&page))
            .fetch_all(&self.pool)
            .await?
        } else {
            let sql = video_query_sql(
                "WHERE (v.title ILIKE $1 OR u.name ILIKE $1)
                 ORDER BY v.created_at DESC, v.id DESC
                 LIMIT $2",
            );
            sqlx::query_as::<_, VideoRecord>(&sql)
            .bind(&like_query)
            .bind(limit_plus_one(&page))
            .fetch_all(&self.pool)
            .await?
        };

        into_page(records, &page, newest_cursor)
    }

    async fn save(&self, video: &Video) -> anyhow::Result<Video> {
        let record = sqlx::query_as::<_, VideoRecord>(
            "INSERT INTO videos (id, user_id, title, description, video_url, thumbnail_url, preview_url, duration_seconds, view_count, like_count, dislike_count)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
             RETURNING
                id,
                user_id,
                (SELECT name FROM users WHERE users.id = videos.user_id) AS username,
                (SELECT profile_picture FROM users WHERE users.id = videos.user_id) AS user_picture,
                title,
                description,
                video_url,
                thumbnail_url,
                preview_url,
                duration_seconds,
                view_count,
                like_count,
                dislike_count,
                created_at",
        )
        .bind(video.id)
        .bind(video.author.id)
        .bind(&video.title)
        .bind(&video.description)
        .bind(video.video_url.to_string())
        .bind(video.thumbnail_url.to_string())
        .bind(video.preview_url.to_string())
        .bind(video.duration_seconds)
        .bind(video.view_count)
        .bind(video.like_count)
        .bind(video.dislike_count)
        .fetch_one(&self.pool)
        .await?;

        record.into_video()
    }
}

#[async_trait::async_trait]
impl VideoReactionRepository for PgVideoRepository {
    async fn find_like_status(&self, user_id: Uuid, video_id: Uuid) -> anyhow::Result<(bool, bool)> {
        let reaction = sqlx::query_scalar::<_, bool>(
            "SELECT is_liked
             FROM video_likes
             WHERE user_id = $1 AND video_id = $2",
        )
        .bind(user_id)
        .bind(video_id)
        .fetch_optional(&self.pool)
        .await?;

        let status = match reaction {
            Some(true) => (true, false),
            Some(false) => (false, true),
            None => (false, false),
        };

        Ok(status)
    }

    async fn add_like(&self, user_id: Uuid, video_id: Uuid) -> anyhow::Result<()> {
        let mut tx = self.pool.begin().await?;

        let existing = sqlx::query_scalar::<_, bool>(
            "SELECT is_liked
             FROM video_likes
             WHERE user_id = $1 AND video_id = $2
             FOR UPDATE",
        )
        .bind(user_id)
        .bind(video_id)
        .fetch_optional(&mut *tx)
        .await?;

        match existing {
            Some(true) => {}
            Some(false) => {
                sqlx::query(
                    "UPDATE video_likes
                     SET is_liked = TRUE
                     WHERE user_id = $1 AND video_id = $2",
                )
                .bind(user_id)
                .bind(video_id)
                .execute(&mut *tx)
                .await?;

                sqlx::query(
                    "UPDATE videos
                     SET like_count = like_count + 1,
                         dislike_count = GREATEST(dislike_count - 1, 0)
                     WHERE id = $1",
                )
                .bind(video_id)
                .execute(&mut *tx)
                .await?;
            }
            None => {
                sqlx::query(
                    "INSERT INTO video_likes (video_id, user_id, is_liked)
                     VALUES ($1, $2, TRUE)",
                )
                .bind(video_id)
                .bind(user_id)
                .execute(&mut *tx)
                .await?;

                sqlx::query(
                    "UPDATE videos
                     SET like_count = like_count + 1
                     WHERE id = $1",
                )
                .bind(video_id)
                .execute(&mut *tx)
                .await?;
            }
        }

        tx.commit().await?;
        Ok(())
    }

    async fn remove_like(&self, user_id: Uuid, video_id: Uuid) -> anyhow::Result<()> {
        let mut tx = self.pool.begin().await?;

        let removed = sqlx::query_scalar::<_, bool>(
            "DELETE FROM video_likes
             WHERE user_id = $1 AND video_id = $2 AND is_liked = TRUE
             RETURNING TRUE",
        )
        .bind(user_id)
        .bind(video_id)
        .fetch_optional(&mut *tx)
        .await?
        .is_some();

        if removed {
            sqlx::query(
                "UPDATE videos
                 SET like_count = GREATEST(like_count - 1, 0)
                 WHERE id = $1",
            )
            .bind(video_id)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    async fn add_dislike(&self, user_id: Uuid, video_id: Uuid) -> anyhow::Result<()> {
        let mut tx = self.pool.begin().await?;

        let existing = sqlx::query_scalar::<_, bool>(
            "SELECT is_liked
             FROM video_likes
             WHERE user_id = $1 AND video_id = $2
             FOR UPDATE",
        )
        .bind(user_id)
        .bind(video_id)
        .fetch_optional(&mut *tx)
        .await?;

        match existing {
            Some(false) => {}
            Some(true) => {
                sqlx::query(
                    "UPDATE video_likes
                     SET is_liked = FALSE
                     WHERE user_id = $1 AND video_id = $2",
                )
                .bind(user_id)
                .bind(video_id)
                .execute(&mut *tx)
                .await?;

                sqlx::query(
                    "UPDATE videos
                     SET dislike_count = dislike_count + 1,
                         like_count = GREATEST(like_count - 1, 0)
                     WHERE id = $1",
                )
                .bind(video_id)
                .execute(&mut *tx)
                .await?;
            }
            None => {
                sqlx::query(
                    "INSERT INTO video_likes (video_id, user_id, is_liked)
                     VALUES ($1, $2, FALSE)",
                )
                .bind(video_id)
                .bind(user_id)
                .execute(&mut *tx)
                .await?;

                sqlx::query(
                    "UPDATE videos
                     SET dislike_count = dislike_count + 1
                     WHERE id = $1",
                )
                .bind(video_id)
                .execute(&mut *tx)
                .await?;
            }
        }

        tx.commit().await?;
        Ok(())
    }

    async fn remove_dislike(&self, user_id: Uuid, video_id: Uuid) -> anyhow::Result<()> {
        let mut tx = self.pool.begin().await?;

        let removed = sqlx::query_scalar::<_, bool>(
            "DELETE FROM video_likes
             WHERE user_id = $1 AND video_id = $2 AND is_liked = FALSE
             RETURNING TRUE",
        )
        .bind(user_id)
        .bind(video_id)
        .fetch_optional(&mut *tx)
        .await?
        .is_some();

        if removed {
            sqlx::query(
                "UPDATE videos
                 SET dislike_count = GREATEST(dislike_count - 1, 0)
                 WHERE id = $1",
            )
            .bind(video_id)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl VideoViewRepository for PgVideoRepository {
    async fn register_view(
        &self,
        video_id: Uuid,
        user_id: Option<Uuid>,
        ip_address: Option<String>,
        recount_after_seconds: i64,
    ) -> anyhow::Result<()> {
        let mut tx = self.pool.begin().await?;
        let should_increment: bool;

        if let Some(ip_address) = ip_address.clone() {
            let existing = sqlx::query_as::<_, (i64, bool)>(
                "SELECT
                    id,
                    (updated_at < NOW() - make_interval(secs => $3)) AS should_count
                 FROM video_views
                 WHERE video_id = $1 AND ip_address = $2::inet
                 ORDER BY updated_at DESC
                 LIMIT 1
                 FOR UPDATE",
            )
            .bind(video_id)
            .bind(&ip_address)
            .bind(recount_after_seconds)
            .fetch_optional(&mut *tx)
            .await?;

            if let Some((view_id, can_recount)) = existing {
                sqlx::query(
                    "UPDATE video_views
                     SET
                        user_id = COALESCE(user_id, $2),
                        ip_address = $3::inet,
                        updated_at = NOW()
                     WHERE id = $1",
                )
                .bind(view_id)
                .bind(user_id)
                .bind(&ip_address)
                .execute(&mut *tx)
                .await?;

                should_increment = can_recount;
            } else {
                sqlx::query(
                    "INSERT INTO video_views (video_id, user_id, ip_address)
                     VALUES ($1, $2, $3::inet)",
                )
                .bind(video_id)
                .bind(user_id)
                .bind(&ip_address)
                .execute(&mut *tx)
                .await?;

                should_increment = true;
            }
        } else if let Some(user_id) = user_id {
            let existing = sqlx::query_as::<_, (i64, bool)>(
                "SELECT
                    id,
                    (updated_at < NOW() - make_interval(secs => $3)) AS should_count
                 FROM video_views
                 WHERE video_id = $1 AND user_id = $2
                 ORDER BY updated_at DESC
                 LIMIT 1
                 FOR UPDATE",
            )
            .bind(video_id)
            .bind(user_id)
            .bind(recount_after_seconds)
            .fetch_optional(&mut *tx)
            .await?;

            if let Some((view_id, can_recount)) = existing {
                sqlx::query(
                    "UPDATE video_views
                     SET updated_at = NOW()
                     WHERE id = $1",
                )
                .bind(view_id)
                .execute(&mut *tx)
                .await?;

                should_increment = can_recount;
            } else {
                sqlx::query(
                    "INSERT INTO video_views (video_id, user_id) VALUES ($1, $2)",
                )
                .bind(video_id)
                .bind(user_id)
                .execute(&mut *tx)
                .await?;

                should_increment = true;
            }
        } else {
            sqlx::query(
                "INSERT INTO video_views (video_id) VALUES ($1)",
            )
            .bind(video_id)
            .execute(&mut *tx)
            .await?;

            should_increment = true;
        }

        if should_increment {
            sqlx::query(
                "UPDATE videos SET view_count = view_count + 1 WHERE id = $1",
            )
            .bind(video_id)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }
}

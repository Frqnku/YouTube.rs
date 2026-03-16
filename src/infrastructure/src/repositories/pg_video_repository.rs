use anyhow::{Context, anyhow};
use domain::_shared::value_objects::Url;
use domain::video::entity::{Video, VideoAuthor};
use domain::video::{PageRequest, VideoHistoryRepository, LikedVideoRepository, VideoPage, VideoRepository};
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
    watched_seconds: Option<i32>,
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
            self.watched_seconds,
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
     viewer_vv.watched_seconds,
    v.view_count,
    v.like_count,
    v.dislike_count,
    v.created_at
 FROM videos v
 JOIN users u ON u.id = v.user_id
 LEFT JOIN video_views viewer_vv ON viewer_vv.video_id = v.id AND viewer_vv.user_id = $1";

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
    async fn find_by_id(&self, id: Uuid, viewer_user_id: Option<Uuid>) -> Option<Video> {
        let sql = video_query_sql("WHERE v.id = $2");
        let record = sqlx::query_as::<_, VideoRecord>(&sql)
        .bind(viewer_user_id)
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .ok()?;

        record.and_then(|rec| rec.into_video().ok())
    }

    async fn list_newest(&self, page: PageRequest, viewer_user_id: Option<Uuid>) -> anyhow::Result<VideoPage> {
        let records = if let Some(cursor) = page.cursor.as_deref() {
            let (created_at, id) = parse_newest_cursor(cursor)?;
            let sql = video_query_sql(
                "WHERE (v.created_at, v.id) < ($2, $3)
                 ORDER BY v.created_at DESC, v.id DESC
                 LIMIT $4",
            );
            sqlx::query_as::<_, VideoRecord>(&sql)
            .bind(viewer_user_id)
            .bind(created_at)
            .bind(id)
            .bind(limit_plus_one(&page))
            .fetch_all(&self.pool)
            .await?
        } else {
            let sql = video_query_sql("ORDER BY v.created_at DESC, v.id DESC LIMIT $2");
            sqlx::query_as::<_, VideoRecord>(&sql)
            .bind(viewer_user_id)
            .bind(limit_plus_one(&page))
            .fetch_all(&self.pool)
            .await?
        };

        into_page(records, &page, newest_cursor)
    }

    async fn list_most_popular(&self, page: PageRequest, viewer_user_id: Option<Uuid>) -> anyhow::Result<VideoPage> {
        let records = if let Some(cursor) = page.cursor.as_deref() {
            let (view_count, id) = parse_popular_cursor(cursor)?;
            let sql = video_query_sql(
                "WHERE (v.view_count, v.id) < ($2, $3)
                 ORDER BY v.view_count DESC, v.id DESC
                 LIMIT $4",
            );
            sqlx::query_as::<_, VideoRecord>(&sql)
            .bind(viewer_user_id)
            .bind(view_count)
            .bind(id)
            .bind(limit_plus_one(&page))
            .fetch_all(&self.pool)
            .await?
        } else {
            let sql = video_query_sql("ORDER BY v.view_count DESC, v.id DESC LIMIT $2");
            sqlx::query_as::<_, VideoRecord>(&sql)
            .bind(viewer_user_id)
            .bind(limit_plus_one(&page))
            .fetch_all(&self.pool)
            .await?
        };

        into_page(records, &page, popular_cursor)
    }

    async fn list_by_user_id(&self, user_id: Uuid, page: PageRequest, viewer_user_id: Option<Uuid>) -> anyhow::Result<VideoPage> {
        let records = if let Some(cursor) = page.cursor.as_deref() {
            let (created_at, id) = parse_newest_cursor(cursor)?;
            let sql = video_query_sql(
                "WHERE v.user_id = $2 AND (v.created_at, v.id) < ($3, $4)
                 ORDER BY v.created_at DESC, v.id DESC
                 LIMIT $5",
            );
            sqlx::query_as::<_, VideoRecord>(&sql)
            .bind(viewer_user_id)
            .bind(user_id)
            .bind(created_at)
            .bind(id)
            .bind(limit_plus_one(&page))
            .fetch_all(&self.pool)
            .await?
        } else {
            let sql = video_query_sql(
                "WHERE v.user_id = $2
                 ORDER BY v.created_at DESC, v.id DESC
                 LIMIT $3",
            );
            sqlx::query_as::<_, VideoRecord>(&sql)
            .bind(viewer_user_id)
            .bind(user_id)
            .bind(limit_plus_one(&page))
            .fetch_all(&self.pool)
            .await?
        };

        into_page(records, &page, newest_cursor)
    }

    async fn search_by_title(&self, query: &str, page: PageRequest, viewer_user_id: Option<Uuid>) -> anyhow::Result<VideoPage> {
        let like_query = format!("%{}%", query);

        let records = if let Some(cursor) = page.cursor.as_deref() {
            let (created_at, id) = parse_newest_cursor(cursor)?;
            let sql = video_query_sql(
                "WHERE (v.title ILIKE $2 OR u.name ILIKE $2) AND (v.created_at, v.id) < ($3, $4)
                 ORDER BY v.created_at DESC, v.id DESC
                 LIMIT $5",
            );
            sqlx::query_as::<_, VideoRecord>(&sql)
            .bind(viewer_user_id)
            .bind(&like_query)
            .bind(created_at)
            .bind(id)
            .bind(limit_plus_one(&page))
            .fetch_all(&self.pool)
            .await?
        } else {
            let sql = video_query_sql(
                "WHERE (v.title ILIKE $2 OR u.name ILIKE $2)
                 ORDER BY v.created_at DESC, v.id DESC
                 LIMIT $3",
            );
            sqlx::query_as::<_, VideoRecord>(&sql)
            .bind(viewer_user_id)
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
                NULL::integer AS watched_seconds,
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
impl VideoHistoryRepository for PgVideoRepository {
    async fn list_history_by_user_id(&self, user_id: Uuid, page: PageRequest) -> anyhow::Result<VideoPage> {
        let records = if let Some(cursor) = page.cursor.as_deref() {
            let (created_at, id) = parse_newest_cursor(cursor)?;
            let sql = video_query_sql(
                "JOIN video_views vv ON vv.video_id = v.id
                 WHERE vv.user_id = $2 AND (vv.updated_at, v.id) < ($3, $4)
                 ORDER BY vv.updated_at DESC, v.id DESC
                 LIMIT $5",
            );
            sqlx::query_as::<_, VideoRecord>(&sql)
            .bind(Some(user_id))
            .bind(user_id)
            .bind(created_at)
            .bind(id)
            .bind(limit_plus_one(&page))
            .fetch_all(&self.pool)
            .await?
        } else {
            let sql = video_query_sql(
                "JOIN video_views vv ON vv.video_id = v.id
                 WHERE vv.user_id = $2
                 ORDER BY vv.updated_at DESC, v.id DESC
                 LIMIT $3",
            );
            sqlx::query_as::<_, VideoRecord>(&sql)
            .bind(Some(user_id))
            .bind(user_id)
            .bind(limit_plus_one(&page))
            .fetch_all(&self.pool)
            .await?
        };

        into_page(records, &page, newest_cursor)
    }
}

#[async_trait::async_trait]
impl LikedVideoRepository for PgVideoRepository {
    async fn list_liked_videos_by_user_id(&self, user_id: Uuid, page: PageRequest) -> anyhow::Result<VideoPage> {
        let records = if let Some(cursor) = page.cursor.as_deref() {
            let (updated_at, id) = parse_newest_cursor(cursor)?;
            let sql = video_query_sql(
                "JOIN video_reactions vr ON vr.video_id = v.id
                 WHERE vr.user_id = $2 AND (vr.updated_at, v.id) < ($3, $4) AND vr.is_liked = true
                 ORDER BY vr.updated_at DESC, v.id DESC
                 LIMIT $5",
            );
            sqlx::query_as::<_, VideoRecord>(&sql)
            .bind(Some(user_id))
            .bind(user_id)
            .bind(updated_at)
            .bind(id)
            .bind(limit_plus_one(&page))
            .fetch_all(&self.pool)
            .await?
        } else {
            let sql = video_query_sql(
                "JOIN video_reactions vr ON vr.video_id = v.id
                 WHERE vr.user_id = $2 AND vr.is_liked = true
                 ORDER BY vr.updated_at DESC, v.id DESC
                 LIMIT $3",
            );
            sqlx::query_as::<_, VideoRecord>(&sql)
            .bind(Some(user_id))
            .bind(user_id)
            .bind(limit_plus_one(&page))
            .fetch_all(&self.pool)
            .await?
        };

        into_page(records, &page, newest_cursor)
    }
}
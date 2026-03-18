use anyhow::Context;
use domain::video::{PageRequest, VideoTagRepository, entity::Tag};
use sqlx::types::Uuid;

pub struct PgVideoTagRepository {
    pool: sqlx::PgPool,
}

impl PgVideoTagRepository {
    pub fn new(pool: &sqlx::PgPool) -> Self {
        Self { pool: pool.clone() }
    }
}

#[derive(sqlx::FromRow)]
struct TagRecord {
    id: i32,
    name: String,
}

impl TagRecord {
    fn into_tag(self) -> Tag {
        Tag::new(self.id, self.name)
    }
}

fn normalize_tag_name(raw: &str) -> Option<String> {
    let compact = raw.split_whitespace().collect::<Vec<_>>().join(" ");
    let normalized = compact.trim().to_lowercase();

    if normalized.is_empty() {
        None
    } else {
        Some(normalized)
    }
}

#[async_trait::async_trait]
impl VideoTagRepository for PgVideoTagRepository {
    async fn list_tags_by_video_id(&self, video_id: Uuid) -> anyhow::Result<Vec<Tag>> {
        let records = sqlx::query_as::<_, TagRecord>(
            "SELECT t.id, t.name
             FROM video_tags vt
             JOIN tags t ON t.id = vt.tag_id
             WHERE vt.video_id = $1
             ORDER BY t.name ASC",
        )
        .bind(video_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(records.into_iter().map(TagRecord::into_tag).collect())
    }

    async fn list_video_ids_by_tag(&self, tag_name: &str, page: PageRequest) -> anyhow::Result<Vec<Uuid>> {
        let normalized = normalize_tag_name(tag_name)
            .context("Tag name cannot be empty")?;

        let offset = page
            .cursor
            .as_deref()
            .map(|cursor| cursor.parse::<i64>().context("Invalid tag page cursor"))
            .transpose()?
            .unwrap_or(0);

        let video_ids = sqlx::query_scalar::<_, Uuid>(
            "SELECT vt.video_id
             FROM video_tags vt
             JOIN tags t ON t.id = vt.tag_id
             WHERE t.name = $1
             ORDER BY vt.created_at DESC, vt.video_id DESC
             LIMIT $2 OFFSET $3",
        )
        .bind(normalized)
        .bind(i64::from(page.limit))
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(video_ids)
    }
}

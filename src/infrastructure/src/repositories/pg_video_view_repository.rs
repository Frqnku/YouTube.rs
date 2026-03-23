use domain::video::VideoViewRepository;
use uuid::Uuid;

pub struct PgVideoViewRepository {
    pool: sqlx::PgPool,
}

impl PgVideoViewRepository {
    pub fn new(pool: &sqlx::PgPool) -> Self {
        Self { pool: pool.clone() }
    }
}

struct ExistingUserViewRecord {
    id: i64,
    should_count: bool,
}

struct ExistingIpViewRecord {
    id: i64,
    user_id: Option<Uuid>,
    should_count: bool,
}

#[async_trait::async_trait]
impl VideoViewRepository for PgVideoViewRepository {
    async fn register_view(
        &self,
        video_id: Uuid,
        user_id: Option<Uuid>,
        ip_address: Option<String>,
        recount_after_seconds: i64,
    ) -> anyhow::Result<()> {
        let mut tx = self.pool.begin().await?;
        let should_increment: bool;

        if let Some(user_id) = user_id {
            let existing_user_view = sqlx::query_as!(
                ExistingUserViewRecord,
                "SELECT
                    id,
                    (updated_at < NOW() - ($3 * INTERVAL '1 second')) AS \"should_count!\"
                 FROM video_views
                 WHERE video_id = $1 AND user_id = $2
                 ORDER BY updated_at DESC
                 LIMIT 1
                 FOR UPDATE",
                video_id,
                user_id,
                recount_after_seconds as f64,
            )
            .fetch_optional(&mut *tx)
            .await?;

            let existing_ip_view = if let Some(ip_address) = ip_address.as_deref() {
                sqlx::query_as!(
                    ExistingIpViewRecord,
                    "SELECT
                        id,
                        user_id,
                                (updated_at < NOW() - ($3 * INTERVAL '1 second')) AS \"should_count!\"
                     FROM video_views
                            WHERE video_id = $1 AND ip_address = $2::text::inet
                     ORDER BY updated_at DESC
                     LIMIT 1
                     FOR UPDATE",
                    video_id,
                    ip_address,
                    recount_after_seconds as f64,
                )
                .fetch_optional(&mut *tx)
                .await?
            } else {
                None
            };

            let ip_can_recount = existing_ip_view
                .as_ref()
                .map(|row| row.should_count)
                .unwrap_or(true);

            if let Some(existing_user_view) = existing_user_view {
                sqlx::query(
                    "UPDATE video_views
                     SET
                        ip_address = COALESCE($2::inet, ip_address),
                        updated_at = NOW()
                     WHERE id = $1",
                )
                .bind(existing_user_view.id)
                .bind(ip_address.as_deref())
                .execute(&mut *tx)
                .await?;

                should_increment = existing_user_view.should_count && ip_can_recount;
            } else {
                if let Some(existing_ip_view) = existing_ip_view {
                    if existing_ip_view.user_id.is_none() {
                        sqlx::query(
                            "UPDATE video_views
                             SET
                                user_id = $2,
                                ip_address = COALESCE($3::inet, ip_address),
                                updated_at = NOW()
                             WHERE id = $1",
                        )
                        .bind(existing_ip_view.id)
                        .bind(user_id)
                        .bind(ip_address.as_deref())
                        .execute(&mut *tx)
                        .await?;
                    } else {
                        sqlx::query(
                            "INSERT INTO video_views (video_id, user_id, ip_address)
                             VALUES ($1, $2, $3::inet)
                             ON CONFLICT (video_id, user_id)
                             DO UPDATE
                             SET
                                ip_address = COALESCE(EXCLUDED.ip_address, video_views.ip_address),
                                updated_at = NOW()",
                        )
                        .bind(video_id)
                        .bind(user_id)
                        .bind(ip_address.as_deref())
                        .execute(&mut *tx)
                        .await?;
                    }
                } else {
                    sqlx::query(
                        "INSERT INTO video_views (video_id, user_id, ip_address)
                         VALUES ($1, $2, $3::inet)
                         ON CONFLICT (video_id, user_id)
                         DO UPDATE
                         SET
                            ip_address = COALESCE(EXCLUDED.ip_address, video_views.ip_address),
                            updated_at = NOW()",
                    )
                    .bind(video_id)
                    .bind(user_id)
                    .bind(ip_address.as_deref())
                    .execute(&mut *tx)
                    .await?;
                }

                should_increment = ip_can_recount;
            }
        } else if let Some(ip_address) = ip_address {
            let existing = sqlx::query_as!(
                ExistingUserViewRecord,
                "SELECT
                    id,
                    (updated_at < NOW() - ($3 * INTERVAL '1 second')) AS \"should_count!\"
                 FROM video_views
                 WHERE video_id = $1 AND ip_address = $2::text::inet
                 ORDER BY updated_at DESC
                 LIMIT 1
                 FOR UPDATE",
                video_id,
                &ip_address,
                recount_after_seconds as f64,
            )
            .fetch_optional(&mut *tx)
            .await?;

            if let Some(existing) = existing {
                sqlx::query(
                    "UPDATE video_views
                     SET
                        ip_address = $2::inet,
                        updated_at = NOW()
                     WHERE id = $1",
                )
                .bind(existing.id)
                .bind(&ip_address)
                .execute(&mut *tx)
                .await?;

                should_increment = existing.should_count;
            } else {
                sqlx::query(
                    "INSERT INTO video_views (video_id, ip_address) VALUES ($1, $2::inet)",
                )
                .bind(video_id)
                .bind(&ip_address)
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

    async fn update_watched_seconds(&self, video_id: Uuid, user_id: Uuid, watched_seconds: u32) -> anyhow::Result<()> {
        sqlx::query(
            "UPDATE video_views
             SET watched_seconds = $3, updated_at = NOW()
             WHERE video_id = $1 AND user_id = $2",
        )
        .bind(video_id)
        .bind(user_id)
        .bind(watched_seconds as i32)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
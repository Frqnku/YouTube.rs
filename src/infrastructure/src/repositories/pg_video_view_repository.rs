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
            let existing_user_view = sqlx::query_as::<_, (i64, bool)>(
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

            let existing_ip_view = if let Some(ip_address) = ip_address.as_deref() {
                sqlx::query_as::<_, (i64, Option<Uuid>, bool)>(
                    "SELECT
                        id,
                        user_id,
                        (updated_at < NOW() - make_interval(secs => $3)) AS should_count
                     FROM video_views
                     WHERE video_id = $1 AND ip_address = $2::inet
                     ORDER BY updated_at DESC
                     LIMIT 1
                     FOR UPDATE",
                )
                .bind(video_id)
                .bind(ip_address)
                .bind(recount_after_seconds)
                .fetch_optional(&mut *tx)
                .await?
            } else {
                None
            };

            let ip_can_recount = existing_ip_view
                .as_ref()
                .map(|(_, _, can_recount)| *can_recount)
                .unwrap_or(true);

            if let Some((view_id, user_can_recount)) = existing_user_view {
                sqlx::query(
                    "UPDATE video_views
                     SET
                        ip_address = COALESCE($2::inet, ip_address),
                        updated_at = NOW()
                     WHERE id = $1",
                )
                .bind(view_id)
                .bind(ip_address.as_deref())
                .execute(&mut *tx)
                .await?;

                should_increment = user_can_recount && ip_can_recount;
            } else {
                if let Some((ip_view_id, ip_user_id, _)) = existing_ip_view {
                    if ip_user_id.is_none() {
                        sqlx::query(
                            "UPDATE video_views
                             SET
                                user_id = $2,
                                ip_address = COALESCE($3::inet, ip_address),
                                updated_at = NOW()
                             WHERE id = $1",
                        )
                        .bind(ip_view_id)
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
                        ip_address = $2::inet,
                        updated_at = NOW()
                     WHERE id = $1",
                )
                .bind(view_id)
                .bind(&ip_address)
                .execute(&mut *tx)
                .await?;

                should_increment = can_recount;
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
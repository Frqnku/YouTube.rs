use domain::video::VideoReactionRepository;
use uuid::Uuid;

pub struct PgVideoReactionRepository {
    pool: sqlx::PgPool,
}

impl PgVideoReactionRepository {
    pub fn new(pool: &sqlx::PgPool) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait::async_trait]
impl VideoReactionRepository for PgVideoReactionRepository {
    async fn find_like_status(&self, user_id: Uuid, video_id: Uuid) -> anyhow::Result<(bool, bool)> {
        let reaction = sqlx::query_scalar::<_, bool>(
            "SELECT is_liked
             FROM video_reactions
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
             FROM video_reactions
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
                    "UPDATE video_reactions
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
                    "INSERT INTO video_reactions (video_id, user_id, is_liked)
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
            "DELETE FROM video_reactions
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
             FROM video_reactions
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
                    "UPDATE video_reactions
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
                    "INSERT INTO video_reactions (video_id, user_id, is_liked)
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
            "DELETE FROM video_reactions
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
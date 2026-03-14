use crate::dtos::VideoCardPage;
use domain::{_shared::DomainError, video::{PageRequest, LikedVideoRepository}};

pub struct ListLikedVideos<
    'a,
    V: LikedVideoRepository
> {
    pub video_repository: &'a V,
}

impl<'a, V> ListLikedVideos<'a, V>
where
    V: LikedVideoRepository,
{
    pub async fn execute(&self, user_id: String, limit: u32, cursor: Option<String>) -> anyhow::Result<VideoCardPage> {
        let id = uuid::Uuid::parse_str(&user_id)
            .map_err(|_| DomainError::VideoNotFound)?;

        let page = self.video_repository
            .list_liked_videos_by_user_id(id, PageRequest::new(limit, cursor))
            .await?;

        Ok(page.into())
    }
}
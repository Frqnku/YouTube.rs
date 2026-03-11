use crate::dtos::{VideoPlayer, VideoCardPage};
use domain::{_shared::DomainError, video::{PageRequest, VideoRepository}};

pub struct ListVideos<
    'a,
    V: VideoRepository
> {
    pub video_repository: &'a V,
}

impl<'a, V> ListVideos<'a, V>
where
    V: VideoRepository,
{
    pub async fn by_newest(&self, limit: u32, cursor: Option<String>) -> anyhow::Result<VideoCardPage> {
        let page = self.video_repository
            .list_newest(PageRequest::new(limit, cursor))
            .await?;

        Ok(page.into())
    }

    pub async fn by_most_popular(&self, limit: u32, cursor: Option<String>) -> anyhow::Result<VideoCardPage> {
        let page = self.video_repository
            .list_most_popular(PageRequest::new(limit, cursor))
            .await?;

        Ok(page.into())
    }

    pub async fn by_user_id(&self, user_id: String, limit: u32, cursor: Option<String>) -> anyhow::Result<VideoCardPage> {
        let id = uuid::Uuid::parse_str(&user_id)
            .map_err(|_| DomainError::VideoNotFound)?;

        let page = self.video_repository
            .list_by_user_id(id, PageRequest::new(limit, cursor))
            .await?;

        Ok(page.into())
    }

    pub async fn by_title_regex(&self, query: &str, limit: u32, cursor: Option<String>) -> anyhow::Result<VideoCardPage> {
        let page = self.video_repository
            .search_by_title(query, PageRequest::new(limit, cursor))
            .await?;

        Ok(page.into())
    }
}

pub struct GetVideoById<
    'a,
    V: VideoRepository
> {
    pub video_repository: &'a V,
}

impl<'a, V> GetVideoById<'a, V>
where
    V: VideoRepository,
{
    pub async fn execute(&self, id: String) -> anyhow::Result<Option<VideoPlayer>> {
        let id = uuid::Uuid::parse_str(&id)
            .map_err(|_| DomainError::VideoNotFound)?;
        let video = self.video_repository.find_by_id(id).await;

        Ok(video.map(Into::into))
    }
}
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::_tests::repositories::InMemoryVideoRepository;
    use domain::_shared::value_objects::Url;
    use domain::video::entity::{Video, VideoAuthor};

    fn make_video(
        author_id: uuid::Uuid,
        author_name: &str,
        title: &str,
        view_count: i64,
        created_at: chrono::DateTime<chrono::Utc>,
    ) -> Video {
        let mut video = Video::new(
            uuid::Uuid::new_v4(),
            VideoAuthor::new(author_id, author_name.to_string(), None),
            title.to_string(),
            "description".to_string(),
            Url::try_from("https://example.com/video.mp4").unwrap(),
            Url::try_from("https://example.com/thumb.jpg").unwrap(),
            Url::try_from("https://example.com/preview.mp4").unwrap(),
            42,
            view_count,
            10,
            1,
        );
        video.created_at = created_at;
        video
    }

    #[tokio::test]
    async fn test_list_videos_by_newest() {
        let repository = InMemoryVideoRepository::new();
        let use_case = ListVideos {
            video_repository: &repository,
        };

        let author_id = uuid::Uuid::new_v4();
        let older = make_video(
            author_id,
            "Alice",
            "older",
            10,
            chrono::Utc::now() - chrono::Duration::minutes(2),
        );
        let newer = make_video(author_id, "Alice", "newer", 20, chrono::Utc::now());

        repository.save(&older).await.unwrap();
        repository.save(&newer).await.unwrap();

        let page = use_case.by_newest(10, None).await.unwrap();

        assert_eq!(page.items.len(), 2);
        assert_eq!(page.items[0].title, "newer");
        assert_eq!(page.items[1].title, "older");
    }

    #[tokio::test]
    async fn test_list_videos_by_most_popular() {
        let repository = InMemoryVideoRepository::new();
        let use_case = ListVideos {
            video_repository: &repository,
        };

        let author_id = uuid::Uuid::new_v4();
        let low = make_video(author_id, "Alice", "low", 10, chrono::Utc::now());
        let high = make_video(author_id, "Alice", "high", 1000, chrono::Utc::now());

        repository.save(&low).await.unwrap();
        repository.save(&high).await.unwrap();

        let page = use_case.by_most_popular(10, None).await.unwrap();

        assert_eq!(page.items.len(), 2);
        assert_eq!(page.items[0].title, "high");
        assert_eq!(page.items[1].title, "low");
    }

    #[tokio::test]
    async fn test_list_videos_by_user_id() {
        let repository = InMemoryVideoRepository::new();
        let use_case = ListVideos {
            video_repository: &repository,
        };

        let wanted_author = uuid::Uuid::new_v4();
        let other_author = uuid::Uuid::new_v4();
        let mine = make_video(wanted_author, "Bob", "my-video", 10, chrono::Utc::now());
        let other = make_video(other_author, "Eve", "other-video", 20, chrono::Utc::now());

        repository.save(&mine).await.unwrap();
        repository.save(&other).await.unwrap();

        let page = use_case
            .by_user_id(wanted_author.to_string(), 10, None)
            .await
            .unwrap();

        assert_eq!(page.items.len(), 1);
        assert_eq!(page.items[0].title, "my-video");
    }

    #[tokio::test]
    async fn test_list_videos_by_title_regex() {
        let repository = InMemoryVideoRepository::new();
        let use_case = ListVideos {
            video_repository: &repository,
        };

        let author_id = uuid::Uuid::new_v4();
        let rust_video = make_video(author_id, "Alice", "Learn Rust", 10, chrono::Utc::now());
        let js_video = make_video(author_id, "Alice", "Learn JS", 10, chrono::Utc::now());

        repository.save(&rust_video).await.unwrap();
        repository.save(&js_video).await.unwrap();

        let page = use_case.by_title_regex("rust", 10, None).await.unwrap();

        assert_eq!(page.items.len(), 1);
        assert_eq!(page.items[0].title, "Learn Rust");
    }

    #[tokio::test]
    async fn test_get_video_by_id_success() {
        let repository = InMemoryVideoRepository::new();
        let use_case = GetVideoById {
            video_repository: &repository,
        };

        let author_id = uuid::Uuid::new_v4();
        let video = make_video(author_id, "Alice", "target-video", 10, chrono::Utc::now());
        let video_id = video.id;
        repository.save(&video).await.unwrap();

        let result = use_case.execute(video_id.to_string()).await.unwrap();

        assert!(result.is_some());
        assert_eq!(result.unwrap().title, "target-video");
    }

    #[tokio::test]
    async fn test_get_video_by_id_invalid_uuid() {
        let repository = InMemoryVideoRepository::new();
        let use_case = GetVideoById {
            video_repository: &repository,
        };

        let result = use_case.execute("invalid".to_string()).await;

        assert!(result.is_err());
    }
}
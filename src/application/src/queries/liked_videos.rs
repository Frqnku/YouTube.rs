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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::_tests::repositories::{InMemoryVideoRepository, LikedVideoEntry};
    use domain::_shared::value_objects::Url;
    use domain::video::VideoRepository;
    use domain::video::entity::{Video, VideoAuthor};

    fn make_video(author_id: uuid::Uuid, title: &str) -> Video {
        Video::new(
            uuid::Uuid::new_v4(),
            VideoAuthor::new(author_id, "Author".to_string(), None),
            title.to_string(),
            "description".to_string(),
            Url::try_from("https://example.com/video.mp4").unwrap(),
            Url::try_from("https://example.com/thumb.jpg").unwrap(),
            Url::try_from("https://example.com/preview.mp4").unwrap(),
            60,
            Some(18),
            0,
            0,
            0,
        )
    }

    #[tokio::test]
    async fn test_list_liked_videos_success() {
        let repository = InMemoryVideoRepository::new();
        let use_case = ListLikedVideos {
            video_repository: &repository,
        };

        let user_id = uuid::Uuid::new_v4();
        let author_id = uuid::Uuid::new_v4();

        let video_a = make_video(author_id, "Liked A");
        let video_b = make_video(author_id, "Liked B");
        repository.save(&video_a).await.unwrap();
        repository.save(&video_b).await.unwrap();

        repository.liked_video_entries.lock().unwrap().push(LikedVideoEntry {
            user_id,
            video_id: video_a.id,
            updated_at: chrono::Utc::now() - chrono::Duration::minutes(1),
        });
        repository.liked_video_entries.lock().unwrap().push(LikedVideoEntry {
            user_id,
            video_id: video_b.id,
            updated_at: chrono::Utc::now(),
        });

        let result = use_case.execute(user_id.to_string(), 10, None).await.unwrap();

        assert_eq!(result.items.len(), 2);
        assert_eq!(result.items[0].title, "Liked B");
        assert_eq!(result.items[1].title, "Liked A");
    }

    #[tokio::test]
    async fn test_list_liked_videos_invalid_user_id() {
        let repository = InMemoryVideoRepository::new();
        let use_case = ListLikedVideos {
            video_repository: &repository,
        };

        let result = use_case.execute("invalid-user".to_string(), 10, None).await;

        assert!(result.is_err());
    }
}
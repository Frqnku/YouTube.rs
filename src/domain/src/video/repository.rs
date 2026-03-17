use uuid::Uuid;

use crate::video::entity::{Tag, Video};

pub const DEFAULT_PAGE_LIMIT: u32 = 6;
pub const MAX_PAGE_LIMIT: u32 = 50;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageRequest {
    pub limit: u32,
    pub cursor: Option<String>,
}

impl PageRequest {
    pub fn new(limit: u32, cursor: Option<String>) -> Self {
        let bounded_limit = if limit == 0 {
            DEFAULT_PAGE_LIMIT
        } else {
            limit.min(MAX_PAGE_LIMIT)
        };

        Self {
            limit: bounded_limit,
            cursor,
        }
    }
}

impl Default for PageRequest {
    fn default() -> Self {
        Self::new(
            DEFAULT_PAGE_LIMIT,
            None,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VideoPage {
    pub items: Vec<Video>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
}

impl VideoPage {
    pub fn new(
        items: Vec<Video>,
        next_cursor: Option<String>,
        has_more: bool,
    ) -> Self {
        Self {
            items,
            next_cursor,
            has_more,
        }
    }
}

#[async_trait::async_trait]
pub trait VideoRepository {
    async fn find_by_id(&self, id: Uuid, viewer_user_id: Option<Uuid>) -> Option<Video>;
    async fn list_newest(&self, page: PageRequest, viewer_user_id: Option<Uuid>) -> anyhow::Result<VideoPage>;
    async fn list_most_popular(&self, page: PageRequest, viewer_user_id: Option<Uuid>) -> anyhow::Result<VideoPage>;
    async fn list_random(&self, page: PageRequest, exclude_video_id: Option<Uuid>, viewer_user_id: Option<Uuid>) -> anyhow::Result<VideoPage>;
    async fn list_by_user_id(&self, user_id: Uuid, page: PageRequest, viewer_user_id: Option<Uuid>) -> anyhow::Result<VideoPage>;
    async fn list_by_tag(&self, tag_name: &str, page: PageRequest, viewer_user_id: Option<Uuid>) -> anyhow::Result<VideoPage>;
    async fn count_by_user_id(&self, user_id: Uuid) -> anyhow::Result<u64>;
    async fn search_by_title(&self, query: &str, page: PageRequest, viewer_user_id: Option<Uuid>) -> anyhow::Result<VideoPage>;
    async fn save(&self, video: &Video) -> anyhow::Result<Video>;
}

#[async_trait::async_trait]
pub trait VideoHistoryRepository {
    async fn list_history_by_user_id(&self, user_id: Uuid, page: PageRequest) -> anyhow::Result<VideoPage>;
}

#[async_trait::async_trait]
pub trait LikedVideoRepository {
    async fn list_liked_videos_by_user_id(&self, user_id: Uuid, page: PageRequest) -> anyhow::Result<VideoPage>;
}

#[async_trait::async_trait]
pub trait VideoReactionRepository {
    async fn find_like_status(&self, user_id: Uuid, video_id: Uuid) -> anyhow::Result<(bool, bool)>;
    async fn add_like(&self, user_id: Uuid, video_id: Uuid) -> anyhow::Result<()>;
    async fn remove_like(&self, user_id: Uuid, video_id: Uuid) -> anyhow::Result<()>;
    async fn add_dislike(&self, user_id: Uuid, video_id: Uuid) -> anyhow::Result<()>;
    async fn remove_dislike(&self, user_id: Uuid, video_id: Uuid) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
pub trait VideoViewRepository {
    async fn register_view(
        &self,
        video_id: Uuid,
        user_id: Option<Uuid>,
        ip_address: Option<String>,
        recount_after_seconds: i64,
    ) -> anyhow::Result<()>;

    async fn update_watched_seconds(
        &self,
        video_id: Uuid,
        user_id: Uuid,
        watched_seconds: u32,
    ) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
pub trait VideoTagRepository {
    async fn list_tags_by_video_id(&self, video_id: Uuid) -> anyhow::Result<Vec<Tag>>;
    async fn list_video_ids_by_tag(&self, tag_name: &str, page: PageRequest) -> anyhow::Result<Vec<Uuid>>;
}
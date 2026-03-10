use uuid::Uuid;

use crate::video::entity::Video;

pub const DEFAULT_PAGE_LIMIT: u32 = 12;
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
        Self {
            limit: DEFAULT_PAGE_LIMIT,
            cursor: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VideoPage {
    pub items: Vec<Video>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
}

#[async_trait::async_trait]
pub trait VideoRepository {
    async fn find_by_id(&self, id: Uuid) -> Option<Video>;
    async fn list_newest(&self, page: PageRequest) -> anyhow::Result<VideoPage>;
    async fn list_most_popular(&self, page: PageRequest) -> anyhow::Result<VideoPage>;
    async fn list_by_user_id(&self, user_id: Uuid, page: PageRequest) -> anyhow::Result<VideoPage>;
    async fn search_by_title(&self, query: &str, page: PageRequest) -> anyhow::Result<VideoPage>;
    async fn save(&self, video: &Video) -> anyhow::Result<Video>;
}
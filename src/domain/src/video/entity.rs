use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::_shared::value_objects::Url;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

impl Tag {
    pub fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VideoAuthor {
    pub id: Uuid,
    pub name: String,
    pub profile_picture: Option<Url>,
}

impl VideoAuthor {
    pub fn new(id: Uuid, name: String, profile_picture: Option<Url>) -> Self {
        Self { id, name, profile_picture }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Video {
    pub id: Uuid,
    pub author: VideoAuthor,
    pub tags: Vec<Tag>,
    pub title: String,
    pub description: String,
    pub video_url: Url,
    pub thumbnail_url: Url,
    pub preview_url: Url,
    pub duration_seconds: i32,
    pub watched_seconds: Option<i32>,
    pub view_count: i64,
    pub like_count: i64,
    pub dislike_count: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Video {
    pub fn new(
        id: Uuid,
        author: VideoAuthor,
        title: String,
        description: String,
        video_url: Url,
        thumbnail_url: Url,
        preview_url: Url,
        duration_seconds: i32,
        watched_seconds: Option<i32>,
        view_count: i64,
        like_count: i64,
        dislike_count: i64,
    ) -> Self {
        Self {
            id,
            author,
            tags: Vec::new(),
            title,
            description,
            video_url,
            thumbnail_url,
            preview_url,
            duration_seconds,
            watched_seconds,
            view_count,
            like_count,
            dislike_count,
            created_at: chrono::Utc::now(),
        }
    }
}
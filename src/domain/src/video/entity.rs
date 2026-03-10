use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::_shared::value_objects::Url;

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
    pub title: String,
    pub description: String,
    pub video_url: Url,
    pub thumbnail_url: Url,
    pub duration_seconds: i32,
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
        duration_seconds: i32,
        view_count: i64,
        like_count: i64,
        dislike_count: i64,
    ) -> Self {
        Self {
            id,
            author,
            title,
            description,
            video_url,
            thumbnail_url,
            duration_seconds,
            view_count,
            like_count,
            dislike_count,
            created_at: chrono::Utc::now(),
        }
    }
}
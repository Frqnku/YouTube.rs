use domain::video::{VideoPage, entity::Video};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoCard {
	pub id: Uuid,
	pub user: String,
	pub user_picture: Option<String>,
	pub title: String,
	pub thumbnail_url: String,
	pub duration_seconds: i32,
	pub view_count: i64,
	pub like_count: i64,
	pub uploaded_at: String,
}

impl From<Video> for VideoCard {
	fn from(video: Video) -> Self {
		Self {
			id: video.id,
			user: video.author.name,
			user_picture: video.author.profile_picture.map(|url| url.to_string()),
			title: video.title,
			thumbnail_url: video.thumbnail_url.to_string(),
			duration_seconds: video.duration_seconds,
			view_count: video.view_count,
			like_count: video.like_count,
			uploaded_at: video.created_at.to_rfc3339(),
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoCardPage {
	pub items: Vec<VideoCard>,
	pub next_cursor: Option<String>,
	pub has_more: bool,
}

impl From<VideoPage> for VideoCardPage {
	fn from(page: VideoPage) -> Self {
		Self {
			items: page.items.into_iter().map(VideoCard::from).collect(),
			next_cursor: page.next_cursor,
			has_more: page.has_more,
		}
	}
}

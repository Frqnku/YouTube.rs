use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use application::dtos::video::VideoCardPage as AppVideoCardPage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoCardDto {
	pub id: String,
	pub user: String,
	pub user_picture: Option<String>,
	pub title: String,
	pub thumbnail_url: String,
	pub duration_seconds: i32,
	pub view_count: i64,
	pub like_count: i64,
	pub uploaded_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoCardPage {
	pub items: Vec<VideoCardDto>,
	pub next_cursor: Option<String>,
	pub has_more: bool,
}

#[cfg(feature = "ssr")]
impl From<AppVideoCardPage> for VideoCardPage {
	fn from(value: AppVideoCardPage) -> Self {
		Self {
			items: value
				.items
				.into_iter()
				.map(|video| VideoCardDto {
					id: video.id.to_string(),
					user: video.user,
					user_picture: video.user_picture,
					title: video.title,
					thumbnail_url: video.thumbnail_url,
					duration_seconds: video.duration_seconds,
					view_count: video.view_count,
					like_count: video.like_count,
					uploaded_at: video.uploaded_at,
				})
				.collect(),
			next_cursor: value.next_cursor,
			has_more: value.has_more,
		}
	}
}

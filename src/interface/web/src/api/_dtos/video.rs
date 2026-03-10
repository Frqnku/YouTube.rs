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

impl VideoCardDto {
	pub fn new(
		id: String,
		user: String,
		user_picture: Option<String>,
		title: String,
		thumbnail_url: String,
		duration_seconds: i32,
		view_count: i64,
		like_count: i64,
		uploaded_at: String,
	) -> Self {
		Self {
			id,
			user,
			user_picture,
			title,
			thumbnail_url,
			duration_seconds,
			view_count,
			like_count,
			uploaded_at,
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoCardPage {
	pub items: Vec<VideoCardDto>,
	pub next_cursor: Option<String>,
	pub has_more: bool,
}

impl VideoCardPage {
	pub fn new(
		items: Vec<VideoCardDto>,
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

#[cfg(feature = "ssr")]
impl From<AppVideoCardPage> for VideoCardPage {
	fn from(value: AppVideoCardPage) -> Self {
		Self::new(
			value
				.items
				.into_iter()
				.map(|video| {
					VideoCardDto::new(
						video.id.to_string(),
						video.user,
						video.user_picture,
						video.title,
						video.thumbnail_url,
						video.duration_seconds,
						video.view_count,
						video.like_count,
						video.uploaded_at,
					)
				})
				.collect(),
			value.next_cursor,
			value.has_more,
		)
	}
}

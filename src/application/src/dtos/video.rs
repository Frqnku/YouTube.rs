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
	pub preview_url: String,
	pub duration_milliseconds: i32,
	pub view_count: i64,
	pub like_count: i64,
	pub uploaded_at: String,
}

impl VideoCard {
	pub fn new(
		id: Uuid,
		user: String,
		user_picture: Option<String>,
		title: String,
		thumbnail_url: String,
		preview_url: String,
		duration_milliseconds: i32,
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
			preview_url,
			duration_milliseconds,
			view_count,
			like_count,
			uploaded_at,
		}
	}
}

impl From<Video> for VideoCard {
	fn from(video: Video) -> Self {
		Self::new(
			video.id,
			video.author.name,
			video.author.profile_picture.map(|url| url.to_string()),
			video.title,
			video.thumbnail_url.to_string(),
			video.preview_url.to_string(),
			video.duration_milliseconds,
			video.view_count,
			video.like_count,
			video.created_at.to_rfc3339(),
		)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoCardPage {
	pub items: Vec<VideoCard>,
	pub next_cursor: Option<String>,
	pub has_more: bool,
}

impl VideoCardPage {
	pub fn new(
		items: Vec<VideoCard>,
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

impl From<VideoPage> for VideoCardPage {
	fn from(page: VideoPage) -> Self {
		Self::new(
			page.items.into_iter().map(VideoCard::from).collect(),
			page.next_cursor,
			page.has_more,
		)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoPlayer {
	pub id: Uuid,
	pub user: String,
	pub user_picture: Option<String>,
	pub title: String,
	pub description: String,
	pub video_url: String,
	pub view_count: i64,
	pub like_count: i64,
	pub dislike_count: i64,
	pub uploaded_at: String,
}

impl VideoPlayer {
	pub fn new(
		id: Uuid,
		user: String,
		user_picture: Option<String>,
		title: String,
		description: String,
		video_url: String,
		view_count: i64,
		like_count: i64,
		dislike_count: i64,
		uploaded_at: String,
	) -> Self {
		Self {
			id,
			user,
			user_picture,
			title,
			description,
			video_url,
			view_count,
			like_count,
			dislike_count,
			uploaded_at,
		}
	}
}

impl From<Video> for VideoPlayer {
	fn from(video: Video) -> Self {
		Self::new(
			video.id,
			video.author.name,
			video.author.profile_picture.map(|url| url.to_string()),
			video.title,
			video.description,
			video.video_url.to_string(),
			video.view_count,
			video.like_count,
			video.dislike_count,
			video.created_at.to_rfc3339(),
		)
	}
}
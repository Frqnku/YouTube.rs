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
	pub duration_seconds: i32,
	pub watched_seconds: Option<i32>,
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
		duration_seconds: i32,
		watched_seconds: Option<i32>,
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
			duration_seconds,
			watched_seconds,
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
			video.duration_seconds,
			video.watched_seconds,
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
	pub duration_seconds: i32,
	pub watched_seconds: Option<i32>,
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
		duration_seconds: i32,
		watched_seconds: Option<i32>,
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
			duration_seconds,
			watched_seconds,
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
			video.duration_seconds,
			video.watched_seconds,
			video.view_count,
			video.like_count,
			video.dislike_count,
			video.created_at.to_rfc3339(),
		)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use domain::_shared::value_objects::Url;
	use domain::video::entity::VideoAuthor;

	fn sample_video(title: &str) -> Video {
		let mut video = Video::new(
			Uuid::new_v4(),
			VideoAuthor::new(
				Uuid::new_v4(),
				"Alice".to_string(),
				Some(Url::try_from("https://example.com/avatar.jpg").unwrap()),
			),
			title.to_string(),
			"A description".to_string(),
			Url::try_from("https://example.com/video.mp4").unwrap(),
			Url::try_from("https://example.com/thumb.jpg").unwrap(),
			Url::try_from("https://example.com/preview.mp4").unwrap(),
			123,
			Some(45),
			777,
			42,
			3,
		);
		video.created_at = chrono::DateTime::parse_from_rfc3339("2026-03-01T10:20:30Z")
			.unwrap()
			.with_timezone(&chrono::Utc);
		video
	}

	#[test]
	fn test_video_card_new() {
		let id = Uuid::new_v4();
		let card = VideoCard::new(
			id,
			"Alice".to_string(),
			Some("https://example.com/avatar.jpg".to_string()),
			"Title".to_string(),
			"https://example.com/thumb.jpg".to_string(),
			"https://example.com/preview.mp4".to_string(),
			123,
			Some(45),
			1000,
			111,
			"2026-03-01T10:20:30Z".to_string(),
		);

		assert_eq!(card.id, id);
		assert_eq!(card.user, "Alice");
		assert_eq!(card.title, "Title");
		assert_eq!(card.view_count, 1000);
	}

	#[test]
	fn test_video_card_from_video() {
		let video = sample_video("Rust Video");
		let expected_uploaded_at = video.created_at.to_rfc3339();

		let card = VideoCard::from(video);

		assert_eq!(card.user, "Alice");
		assert_eq!(card.title, "Rust Video");
		assert_eq!(card.thumbnail_url, "https://example.com/thumb.jpg");
		assert_eq!(card.preview_url, "https://example.com/preview.mp4");
		assert_eq!(card.watched_seconds, Some(45));
		assert_eq!(card.uploaded_at, expected_uploaded_at);
	}

	#[test]
	fn test_video_card_page_from_video_page() {
		let video_a = sample_video("A");
		let video_b = sample_video("B");
		let page = VideoPage::new(vec![video_a, video_b], Some("next-cursor".to_string()), true);

		let card_page = VideoCardPage::from(page);

		assert_eq!(card_page.items.len(), 2);
		assert_eq!(card_page.items[0].title, "A");
		assert_eq!(card_page.items[1].title, "B");
		assert_eq!(card_page.next_cursor.as_deref(), Some("next-cursor"));
		assert!(card_page.has_more);
	}

	#[test]
	fn test_video_player_from_video() {
		let video = sample_video("Player Title");
		let expected_uploaded_at = video.created_at.to_rfc3339();

		let player = VideoPlayer::from(video);

		assert_eq!(player.user, "Alice");
		assert_eq!(player.title, "Player Title");
		assert_eq!(player.description, "A description");
		assert_eq!(player.video_url, "https://example.com/video.mp4");
		assert_eq!(player.duration_seconds, 123);
		assert_eq!(player.watched_seconds, Some(45));
		assert_eq!(player.dislike_count, 3);
		assert_eq!(player.uploaded_at, expected_uploaded_at);
	}
}
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::_shared::value_objects::Url;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommentAuthor {
	pub id: Uuid,
	pub name: String,
	pub profile_picture: Option<Url>,
}

impl CommentAuthor {
	pub fn new(id: Uuid, name: String, profile_picture: Option<Url>) -> Self {
		Self { id, name, profile_picture }
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Comment {
	pub id: Uuid,
	pub video_id: Uuid,
	pub author: CommentAuthor,
	pub parent_id: Option<Uuid>,
	pub content: String,
	pub like_count: i64,
	pub reply_count: i64,
	pub liked_by_viewer: Option<bool>,
	pub created_at: chrono::DateTime<chrono::Utc>,
	pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Comment {
	pub fn new(
		id: Uuid,
		video_id: Uuid,
		author: CommentAuthor,
		parent_id: Option<Uuid>,
		content: String,
		like_count: i64,
		reply_count: i64,
		liked_by_viewer: Option<bool>,
	) -> Self {
		let now = chrono::Utc::now();

		Self {
			id,
			video_id,
			author,
			parent_id,
			content,
			like_count,
			reply_count,
			liked_by_viewer,
			created_at: now,
			updated_at: now,
		}
	}
}

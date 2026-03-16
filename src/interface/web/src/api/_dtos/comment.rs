use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use domain::comment::entity::Comment as DomainComment;
#[cfg(feature = "ssr")]
use domain::comment::CommentPage as DomainCommentPage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentDto {
	pub id: String,
	pub video_id: String,
	pub user_id: String,
	pub user: String,
	pub user_picture: Option<String>,
	pub parent_id: Option<String>,
	pub content: String,
	pub like_count: i64,
	pub reply_count: i64,
	pub liked_by_viewer: Option<bool>,
	pub created_at: String,
	pub updated_at: String,
}

impl CommentDto {
	pub fn new(
		id: String,
		video_id: String,
		user_id: String,
		user: String,
		user_picture: Option<String>,
		parent_id: Option<String>,
		content: String,
		like_count: i64,
		reply_count: i64,
		liked_by_viewer: Option<bool>,
		created_at: String,
		updated_at: String,
	) -> Self {
		Self {
			id,
			video_id,
			user_id,
			user,
			user_picture,
			parent_id,
			content,
			like_count,
			reply_count,
			liked_by_viewer,
			created_at,
			updated_at,
		}
	}
}

#[cfg(feature = "ssr")]
impl From<DomainComment> for CommentDto {
	fn from(comment: DomainComment) -> Self {
		Self::new(
			comment.id.to_string(),
			comment.video_id.to_string(),
			comment.author.id.to_string(),
			comment.author.name,
			comment.author.profile_picture.map(|url| url.to_string()),
			comment.parent_id.map(|id| id.to_string()),
			comment.content,
			comment.like_count,
			comment.reply_count,
			comment.liked_by_viewer,
			comment.created_at.to_rfc3339(),
			comment.updated_at.to_rfc3339(),
		)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentPageDto {
	pub items: Vec<CommentDto>,
	pub next_cursor: Option<String>,
	pub has_more: bool,
}

impl CommentPageDto {
	pub fn new(items: Vec<CommentDto>, next_cursor: Option<String>, has_more: bool) -> Self {
		Self {
			items,
			next_cursor,
			has_more,
		}
	}
}

#[cfg(feature = "ssr")]
impl From<DomainCommentPage> for CommentPageDto {
	fn from(page: DomainCommentPage) -> Self {
		Self::new(
			page.items.into_iter().map(CommentDto::from).collect(),
			page.next_cursor,
			page.has_more,
		)
	}
}

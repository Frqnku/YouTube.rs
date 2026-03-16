use uuid::Uuid;

use crate::comment::entity::Comment;

pub const DEFAULT_COMMENT_PAGE_LIMIT: u32 = 20;
pub const MAX_COMMENT_PAGE_LIMIT: u32 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommentSort {
	Newest,
	Oldest,
	MostLiked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentPageRequest {
	pub limit: u32,
	pub cursor: Option<String>,
	pub sort: CommentSort,
}

impl CommentPageRequest {
	pub fn new(limit: u32, cursor: Option<String>, sort: CommentSort) -> Self {
		let bounded_limit = if limit == 0 {
			DEFAULT_COMMENT_PAGE_LIMIT
		} else {
			limit.min(MAX_COMMENT_PAGE_LIMIT)
		};

		Self {
			limit: bounded_limit,
			cursor,
			sort,
		}
	}
}

impl Default for CommentPageRequest {
	fn default() -> Self {
		Self::new(
			DEFAULT_COMMENT_PAGE_LIMIT,
			None,
			CommentSort::Newest,
		)
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentPage {
	pub items: Vec<Comment>,
	pub next_cursor: Option<String>,
	pub has_more: bool,
}

impl CommentPage {
	pub fn new(items: Vec<Comment>, next_cursor: Option<String>, has_more: bool) -> Self {
		Self {
			items,
			next_cursor,
			has_more,
		}
	}
}

#[async_trait::async_trait]
pub trait CommentRepository {
	async fn find_by_id(&self, id: Uuid, viewer_user_id: Option<Uuid>) -> anyhow::Result<Option<Comment>>;

	async fn list_by_video_id(
		&self,
		video_id: Uuid,
		parent_id: Option<Uuid>,
		page: CommentPageRequest,
		viewer_user_id: Option<Uuid>,
	) -> anyhow::Result<CommentPage>;

	async fn list_replies(
		&self,
		parent_id: Uuid,
		page: CommentPageRequest,
		viewer_user_id: Option<Uuid>,
	) -> anyhow::Result<CommentPage>;

	async fn count_by_video_id(&self, video_id: Uuid) -> anyhow::Result<i64>;
	async fn count_replies(&self, parent_id: Uuid) -> anyhow::Result<i64>;

	async fn save(&self, comment: &Comment) -> anyhow::Result<Comment>;
	async fn update_content(&self, comment_id: Uuid, user_id: Uuid, content: String) -> anyhow::Result<Comment>;
	async fn delete(&self, comment_id: Uuid, user_id: Uuid) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
pub trait CommentLikeRepository {
	async fn is_liked_by_user(&self, comment_id: Uuid, user_id: Uuid) -> anyhow::Result<bool>;
	async fn add_like(&self, comment_id: Uuid, user_id: Uuid) -> anyhow::Result<()>;
	async fn remove_like(&self, comment_id: Uuid, user_id: Uuid) -> anyhow::Result<()>;
}

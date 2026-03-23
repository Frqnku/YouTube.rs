use domain::{
	_shared::DomainError,
	comment::{CommentPage, CommentPageRequest, CommentRepository, CommentSort},
	comment::entity::Comment,
};

use crate::_helpers::parse_uuid;

fn parse_sort(sort: Option<&str>) -> anyhow::Result<CommentSort> {
	match sort.unwrap_or("newest") {
		"newest" => Ok(CommentSort::Newest),
		"oldest" => Ok(CommentSort::Oldest),
		"most-liked" => Ok(CommentSort::MostLiked),
		_ => Err(DomainError::BadRequest("Invalid comment sort".to_string()).into()),
	}
}

pub struct ListVideoComments<
	'a,
	R: CommentRepository,
> {
	pub comment_repository: &'a R,
}

impl<'a, R> ListVideoComments<'a, R>
where
	R: CommentRepository,
{
	pub async fn execute(
		&self,
		video_id: String,
		limit: u32,
		cursor: Option<String>,
		sort: Option<String>,
		viewer_user_id: Option<String>,
	) -> anyhow::Result<CommentPage> {
		let video_id = parse_uuid(&video_id, "video id")?;
		let viewer_user_id = viewer_user_id
			.as_deref()
			.map(|id| parse_uuid(id, "viewer user id"))
			.transpose()?;

		let page = CommentPageRequest::new(limit, cursor, parse_sort(sort.as_deref())?);

		self
			.comment_repository
			.list_by_video_id(video_id, None, page, viewer_user_id)
			.await
	}
}

pub struct ListCommentReplies<
	'a,
	R: CommentRepository,
> {
	pub comment_repository: &'a R,
}

impl<'a, R> ListCommentReplies<'a, R>
where
	R: CommentRepository,
{
	pub async fn execute(
		&self,
		parent_comment_id: String,
		limit: u32,
		cursor: Option<String>,
		sort: Option<String>,
		viewer_user_id: Option<String>,
	) -> anyhow::Result<CommentPage> {
		let parent_comment_id = parse_uuid(&parent_comment_id, "parent comment id")?;
		let viewer_user_id = viewer_user_id
			.as_deref()
			.map(|id| parse_uuid(id, "viewer user id"))
			.transpose()?;

		let page = CommentPageRequest::new(limit, cursor, parse_sort(sort.as_deref())?);

		self
			.comment_repository
			.list_replies(parent_comment_id, page, viewer_user_id)
			.await
	}
}

pub struct GetCommentById<
	'a,
	R: CommentRepository,
> {
	pub comment_repository: &'a R,
}

impl<'a, R> GetCommentById<'a, R>
where
	R: CommentRepository,
{
	pub async fn execute(&self, comment_id: String, viewer_user_id: Option<String>) -> anyhow::Result<Option<Comment>> {
		let comment_id = parse_uuid(&comment_id, "comment id")?;
		let viewer_user_id = viewer_user_id
			.as_deref()
			.map(|id| parse_uuid(id, "viewer user id"))
			.transpose()?;

		self.comment_repository.find_by_id(comment_id, viewer_user_id).await
	}
}

pub struct CountVideoComments<
	'a,
	R: CommentRepository,
> {
	pub comment_repository: &'a R,
}

impl<'a, R> CountVideoComments<'a, R>
where
	R: CommentRepository,
{
	pub async fn execute(&self, video_id: String) -> anyhow::Result<i64> {
		let video_id = parse_uuid(&video_id, "video id")?;
		self.comment_repository.count_by_video_id(video_id).await
	}
}

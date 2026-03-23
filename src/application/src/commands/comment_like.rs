use crate::_helpers::parse_uuid;
use domain::{
	comment::CommentLikeRepository,
};

pub struct AddCommentLike<
	'a,
	R: CommentLikeRepository,
> {
	pub comment_like_repository: &'a R,
}

impl<'a, R> AddCommentLike<'a, R>
where
	R: CommentLikeRepository,
{
	pub async fn execute(&self, comment_id: String, user_id: String) -> anyhow::Result<()> {
		let comment_id = parse_uuid(&comment_id, "comment id")?;
		let user_id = parse_uuid(&user_id, "user id")?;

		self.comment_like_repository
			.add_like(comment_id, user_id)
			.await
	}
}

pub struct RemoveCommentLike<
	'a,
	R: CommentLikeRepository,
> {
	pub comment_like_repository: &'a R,
}

impl<'a, R> RemoveCommentLike<'a, R>
where
	R: CommentLikeRepository,
{
	pub async fn execute(&self, comment_id: String, user_id: String) -> anyhow::Result<()> {
		let comment_id = parse_uuid(&comment_id, "comment id")?;
		let user_id = parse_uuid(&user_id, "user id")?;

		self.comment_like_repository
			.remove_like(comment_id, user_id)
			.await
	}
}

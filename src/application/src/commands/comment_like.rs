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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::_tests::repositories::InMemoryCommentLikeRepository;
	use uuid::Uuid;

	#[tokio::test]
	async fn add_comment_like_calls_repository_with_parsed_ids() {
		let repo = InMemoryCommentLikeRepository::new();
		let command = AddCommentLike {
			comment_like_repository: &repo,
		};

		let comment_id = Uuid::new_v4();
		let user_id = Uuid::new_v4();

		command
			.execute(comment_id.to_string(), user_id.to_string())
			.await
			.unwrap();

		let calls = repo.add_calls.lock().unwrap();
		assert_eq!(calls.len(), 1);
		assert_eq!(calls[0], (comment_id, user_id));
	}

	#[tokio::test]
	async fn add_comment_like_fails_on_invalid_comment_id() {
		let repo = InMemoryCommentLikeRepository::new();
		let command = AddCommentLike {
			comment_like_repository: &repo,
		};

		let result = command
			.execute("not-a-uuid".to_string(), Uuid::new_v4().to_string())
			.await;

		assert!(result.is_err());
		assert!(repo.add_calls.lock().unwrap().is_empty());
	}

	#[tokio::test]
	async fn remove_comment_like_calls_repository_with_parsed_ids() {
		let repo = InMemoryCommentLikeRepository::new();
		let command = RemoveCommentLike {
			comment_like_repository: &repo,
		};

		let comment_id = Uuid::new_v4();
		let user_id = Uuid::new_v4();

		command
			.execute(comment_id.to_string(), user_id.to_string())
			.await
			.unwrap();

		let calls = repo.remove_calls.lock().unwrap();
		assert_eq!(calls.len(), 1);
		assert_eq!(calls[0], (comment_id, user_id));
	}

	#[tokio::test]
	async fn remove_comment_like_fails_on_invalid_user_id() {
		let repo = InMemoryCommentLikeRepository::new();
		let command = RemoveCommentLike {
			comment_like_repository: &repo,
		};

		let result = command
			.execute(Uuid::new_v4().to_string(), "not-a-uuid".to_string())
			.await;

		assert!(result.is_err());
		assert!(repo.remove_calls.lock().unwrap().is_empty());
	}
}

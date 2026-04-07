use crate::_helpers::parse_uuid;
use domain::{
	_shared::DomainError,
	comment::entity::{Comment, CommentAuthor},
	comment::CommentRepository,
};
use uuid::Uuid;

pub struct CreateComment<
	'a,
	R: CommentRepository,
> {
	pub comment_repository: &'a R,
}

impl<'a, R> CreateComment<'a, R>
where
	R: CommentRepository,
{
	pub async fn execute(
		&self,
		video_id: String,
		user_id: String,
		user_name: String,
		user_profile_picture: Option<String>,
		parent_id: Option<String>,
		content: String,
	) -> anyhow::Result<Comment> {
		let video_id = parse_uuid(&video_id, "video id")?;
		let user_id = parse_uuid(&user_id, "user id")?;
		let parent_id = parent_id
			.as_deref()
			.map(|id| parse_uuid(id, "parent comment id"))
			.transpose()?;

		let content = content.trim().to_string();
		if content.is_empty() {
			return Err(DomainError::BadRequest("Comment content cannot be empty".to_string()).into());
		}
		if content.len() > 280 {
			return Err(DomainError::BadRequest("Comment content cannot exceed 280 characters".to_string()).into());
		}

		let user_profile_picture = user_profile_picture
			.map(|value| value.trim().to_string())
			.filter(|value| !value.is_empty())
			.map(domain::_shared::value_objects::Url::try_from)
			.transpose()
			.map_err(anyhow::Error::from)?;

		let comment = Comment::new(
			Uuid::new_v4(),
			video_id,
			CommentAuthor::new(user_id, user_name, user_profile_picture),
			parent_id,
			content,
			0,
			0,
			None,
		);

		self.comment_repository.save(&comment).await
	}
}

pub struct UpdateCommentContent<
	'a,
	R: CommentRepository,
> {
	pub comment_repository: &'a R,
}

impl<'a, R> UpdateCommentContent<'a, R>
where
	R: CommentRepository,
{
	pub async fn execute(&self, comment_id: String, user_id: String, content: String) -> anyhow::Result<Comment> {
		let comment_id = parse_uuid(&comment_id, "comment id")?;
		let user_id = parse_uuid(&user_id, "user id")?;
		let content = content.trim().to_string();

		if content.is_empty() {
			return Err(DomainError::BadRequest("Comment content cannot be empty".to_string()).into());
		}
		if content.len() > 280 {
			return Err(DomainError::BadRequest("Comment content cannot exceed 280 characters".to_string()).into());
		}

		self
			.comment_repository
			.update_content(comment_id, user_id, content)
			.await
	}
}

pub struct DeleteComment<
	'a,
	R: CommentRepository,
> {
	pub comment_repository: &'a R,
}

impl<'a, R> DeleteComment<'a, R>
where
	R: CommentRepository,
{
	pub async fn execute(&self, comment_id: String, user_id: String) -> anyhow::Result<()> {
		let comment_id = parse_uuid(&comment_id, "comment id")?;
		let user_id = parse_uuid(&user_id, "user id")?;

		self.comment_repository.delete(comment_id, user_id).await
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::_tests::repositories::InMemoryCommentRepository;

	#[tokio::test]
	async fn create_comment_trims_and_saves() {
		let repo = InMemoryCommentRepository::new();
		let command = CreateComment {
			comment_repository: &repo,
		};

		let video_id = Uuid::new_v4();
		let user_id = Uuid::new_v4();
		let saved = command
			.execute(
				video_id.to_string(),
				user_id.to_string(),
				"Alice".to_string(),
				Some("  https://example.com/pfp.png  ".to_string()),
				None,
				"  hello world  ".to_string(),
			)
			.await
			.unwrap();

		assert_eq!(saved.video_id, video_id);
		assert_eq!(saved.author.id, user_id);
		assert_eq!(saved.content, "hello world");
		assert_eq!(repo.saved.lock().unwrap().len(), 1);
	}

	#[tokio::test]
	async fn create_comment_rejects_empty_content() {
		let repo = InMemoryCommentRepository::new();
		let command = CreateComment {
			comment_repository: &repo,
		};

		let result = command
			.execute(
				Uuid::new_v4().to_string(),
				Uuid::new_v4().to_string(),
				"Alice".to_string(),
				None,
				None,
				"   ".to_string(),
			)
			.await;

		assert!(result.is_err());
		assert!(repo.saved.lock().unwrap().is_empty());
	}

	#[tokio::test]
	async fn update_comment_content_calls_repository() {
		let repo = InMemoryCommentRepository::new();
		let command = UpdateCommentContent {
			comment_repository: &repo,
		};

		let comment_id = Uuid::new_v4();
		let user_id = Uuid::new_v4();
		let updated = command
			.execute(comment_id.to_string(), user_id.to_string(), "  updated  ".to_string())
			.await
			.unwrap();

		assert_eq!(updated.content, "updated");
		let calls = repo.updated.lock().unwrap();
		assert_eq!(calls.len(), 1);
		assert_eq!(calls[0].0, comment_id);
		assert_eq!(calls[0].1, user_id);
		assert_eq!(calls[0].2, "updated");
	}

	#[tokio::test]
	async fn update_comment_content_rejects_too_long_content() {
		let repo = InMemoryCommentRepository::new();
		let command = UpdateCommentContent {
			comment_repository: &repo,
		};

		let too_long = "a".repeat(281);
		let result = command
			.execute(Uuid::new_v4().to_string(), Uuid::new_v4().to_string(), too_long)
			.await;

		assert!(result.is_err());
		assert!(repo.updated.lock().unwrap().is_empty());
	}

	#[tokio::test]
	async fn delete_comment_calls_repository() {
		let repo = InMemoryCommentRepository::new();
		let command = DeleteComment {
			comment_repository: &repo,
		};

		let comment_id = Uuid::new_v4();
		let user_id = Uuid::new_v4();
		command
			.execute(comment_id.to_string(), user_id.to_string())
			.await
			.unwrap();

		let calls = repo.deleted.lock().unwrap();
		assert_eq!(calls.len(), 1);
		assert_eq!(calls[0], (comment_id, user_id));
	}

	#[tokio::test]
	async fn delete_comment_fails_on_invalid_comment_id() {
		let repo = InMemoryCommentRepository::new();
		let command = DeleteComment {
			comment_repository: &repo,
		};

		let result = command
			.execute("not-a-uuid".to_string(), Uuid::new_v4().to_string())
			.await;

		assert!(result.is_err());
		assert!(repo.deleted.lock().unwrap().is_empty());
	}
}

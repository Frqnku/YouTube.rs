use domain::{
	_shared::DomainError,
	comment::entity::{Comment, CommentAuthor},
	comment::CommentRepository,
};
use uuid::Uuid;

fn parse_uuid(id: &str, field_name: &str) -> anyhow::Result<Uuid> {
	Uuid::parse_str(id)
		.map_err(|_| DomainError::BadRequest(format!("Invalid {field_name}")))
		.map_err(Into::into)
}

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

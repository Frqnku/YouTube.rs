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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::_tests::repositories::InMemoryCommentRepository;
	use domain::comment::{CommentPage, CommentSort};
	use uuid::Uuid;

	#[tokio::test]
	async fn list_video_comments_uses_parsed_args_and_sort() {
		let repo = InMemoryCommentRepository::new();
		*repo.list_result.lock().unwrap() = CommentPage::new(Vec::new(), Some("next".to_string()), true);
		let query = ListVideoComments {
			comment_repository: &repo,
		};

		let video_id = Uuid::new_v4();
		let viewer_id = Uuid::new_v4();
		let page = query
			.execute(
				video_id.to_string(),
				25,
				Some("cursor-1".to_string()),
				Some("oldest".to_string()),
				Some(viewer_id.to_string()),
			)
			.await
			.unwrap();

		assert!(page.has_more);
		let calls = repo.list_calls.lock().unwrap();
		assert_eq!(calls.len(), 1);
		assert_eq!(calls[0].0, video_id);
		assert_eq!(calls[0].2.sort, CommentSort::Oldest);
		assert_eq!(calls[0].3, Some(viewer_id));
	}

	#[tokio::test]
	async fn list_video_comments_rejects_invalid_sort() {
		let repo = InMemoryCommentRepository::new();
		let query = ListVideoComments {
			comment_repository: &repo,
		};

		let result = query
			.execute(
				Uuid::new_v4().to_string(),
				20,
				None,
				Some("wrong-sort".to_string()),
				None,
			)
			.await;

		assert!(result.is_err());
		assert!(repo.list_calls.lock().unwrap().is_empty());
	}

	#[tokio::test]
	async fn list_comment_replies_calls_repository() {
		let repo = InMemoryCommentRepository::new();
		*repo.replies_result.lock().unwrap() = CommentPage::new(Vec::new(), None, false);
		let query = ListCommentReplies {
			comment_repository: &repo,
		};

		let parent_id = Uuid::new_v4();
		query
			.execute(parent_id.to_string(), 0, None, Some("most-liked".to_string()), None)
			.await
			.unwrap();

		let calls = repo.replies_calls.lock().unwrap();
		assert_eq!(calls.len(), 1);
		assert_eq!(calls[0].0, parent_id);
		assert_eq!(calls[0].1.limit, 20);
		assert_eq!(calls[0].1.sort, CommentSort::MostLiked);
	}

	#[tokio::test]
	async fn get_comment_by_id_calls_repository() {
		let repo = InMemoryCommentRepository::new();
		*repo.find_result.lock().unwrap() = Some(Comment::new(
			Uuid::new_v4(),
			Uuid::new_v4(),
			domain::comment::entity::CommentAuthor::new(Uuid::new_v4(), "Alice".to_string(), None),
			None,
			"hello".to_string(),
			1,
			0,
			Some(false),
		));
		let query = GetCommentById {
			comment_repository: &repo,
		};

		let comment_id = Uuid::new_v4();
		let viewer_id = Uuid::new_v4();
		let result = query
			.execute(comment_id.to_string(), Some(viewer_id.to_string()))
			.await
			.unwrap();

		assert!(result.is_some());
		let calls = repo.find_calls.lock().unwrap();
		assert_eq!(calls.len(), 1);
		assert_eq!(calls[0], (comment_id, Some(viewer_id)));
	}

	#[tokio::test]
	async fn count_video_comments_calls_repository() {
		let repo = InMemoryCommentRepository::new();
		*repo.count_result.lock().unwrap() = 17;
		let query = CountVideoComments {
			comment_repository: &repo,
		};

		let video_id = Uuid::new_v4();
		let count = query.execute(video_id.to_string()).await.unwrap();

		assert_eq!(count, 17);
		assert_eq!(repo.count_calls.lock().unwrap()[0], video_id);
	}
}

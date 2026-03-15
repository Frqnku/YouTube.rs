use domain::{
	_shared::DomainError,
	video::VideoReactionRepository,
};
use uuid::Uuid;

fn parse_uuid(id: &str) -> anyhow::Result<Uuid> {
	Uuid::parse_str(id).map_err(|_| DomainError::VideoNotFound.into())
}

pub struct GetVideoReactionStatus<
	'a,
	R: VideoReactionRepository,
> {
	pub reaction_repository: &'a R,
}

impl<'a, R> GetVideoReactionStatus<'a, R>
where
	R: VideoReactionRepository,
{
	pub async fn execute(&self, user_id: String, video_id: String) -> anyhow::Result<(bool, bool)> {
		let user_id = parse_uuid(&user_id)?;
		let video_id = parse_uuid(&video_id)?;

		self.reaction_repository
			.find_like_status(user_id, video_id)
			.await
	}
}

pub struct AddVideoLike<
	'a,
	R: VideoReactionRepository,
> {
	pub reaction_repository: &'a R,
}

impl<'a, R> AddVideoLike<'a, R>
where
	R: VideoReactionRepository,
{
	pub async fn execute(&self, user_id: String, video_id: String) -> anyhow::Result<()> {
		let user_id = parse_uuid(&user_id)?;
		let video_id = parse_uuid(&video_id)?;

		self.reaction_repository
			.add_like(user_id, video_id)
			.await
	}
}

pub struct RemoveVideoLike<
	'a,
	R: VideoReactionRepository,
> {
	pub reaction_repository: &'a R,
}

impl<'a, R> RemoveVideoLike<'a, R>
where
	R: VideoReactionRepository,
{
	pub async fn execute(&self, user_id: String, video_id: String) -> anyhow::Result<()> {
		let user_id = parse_uuid(&user_id)?;
		let video_id = parse_uuid(&video_id)?;

		self.reaction_repository
			.remove_like(user_id, video_id)
			.await
	}
}

pub struct AddVideoDislike<
	'a,
	R: VideoReactionRepository,
> {
	pub reaction_repository: &'a R,
}

impl<'a, R> AddVideoDislike<'a, R>
where
	R: VideoReactionRepository,
{
	pub async fn execute(&self, user_id: String, video_id: String) -> anyhow::Result<()> {
		let user_id = parse_uuid(&user_id)?;
		let video_id = parse_uuid(&video_id)?;

		self.reaction_repository
			.add_dislike(user_id, video_id)
			.await
	}
}

pub struct RemoveVideoDislike<
	'a,
	R: VideoReactionRepository,
> {
	pub reaction_repository: &'a R,
}

impl<'a, R> RemoveVideoDislike<'a, R>
where
	R: VideoReactionRepository,
{
	pub async fn execute(&self, user_id: String, video_id: String) -> anyhow::Result<()> {
		let user_id = parse_uuid(&user_id)?;
		let video_id = parse_uuid(&video_id)?;

		self.reaction_repository
			.remove_dislike(user_id, video_id)
			.await
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::_tests::repositories::InMemoryVideoReactionRepository;

	#[tokio::test]
	async fn test_get_video_reaction_status_success() {
		let repository = InMemoryVideoReactionRepository::new();
		let use_case = GetVideoReactionStatus {
			reaction_repository: &repository,
		};

		let user_id = Uuid::new_v4();
		let video_id = Uuid::new_v4();
		repository.add_like(user_id, video_id).await.unwrap();

		let result = use_case.execute(user_id.to_string(), video_id.to_string()).await;

		assert!(result.is_ok());
		assert_eq!(result.unwrap(), (true, false));
	}

	#[tokio::test]
	async fn test_add_video_like_success() {
		let repository = InMemoryVideoReactionRepository::new();
		let use_case = AddVideoLike {
			reaction_repository: &repository,
		};

		let user_id = Uuid::new_v4();
		let video_id = Uuid::new_v4();

		let result = use_case.execute(user_id.to_string(), video_id.to_string()).await;

		assert!(result.is_ok());
		assert_eq!(repository.find_like_status(user_id, video_id).await.unwrap(), (true, false));
		assert_eq!(repository.like_count(video_id), 1);
	}

	#[tokio::test]
	async fn test_remove_video_like_success() {
		let repository = InMemoryVideoReactionRepository::new();
		let use_case = RemoveVideoLike {
			reaction_repository: &repository,
		};

		let user_id = Uuid::new_v4();
		let video_id = Uuid::new_v4();
		repository.add_like(user_id, video_id).await.unwrap();

		let result = use_case.execute(user_id.to_string(), video_id.to_string()).await;

		assert!(result.is_ok());
		assert_eq!(repository.find_like_status(user_id, video_id).await.unwrap(), (false, false));
		assert_eq!(repository.like_count(video_id), 0);
	}

	#[tokio::test]
	async fn test_add_video_dislike_success() {
		let repository = InMemoryVideoReactionRepository::new();
		let use_case = AddVideoDislike {
			reaction_repository: &repository,
		};

		let user_id = Uuid::new_v4();
		let video_id = Uuid::new_v4();

		let result = use_case.execute(user_id.to_string(), video_id.to_string()).await;

		assert!(result.is_ok());
		assert_eq!(repository.find_like_status(user_id, video_id).await.unwrap(), (false, true));
		assert_eq!(repository.dislike_count(video_id), 1);
	}

	#[tokio::test]
	async fn test_remove_video_dislike_success() {
		let repository = InMemoryVideoReactionRepository::new();
		let use_case = RemoveVideoDislike {
			reaction_repository: &repository,
		};

		let user_id = Uuid::new_v4();
		let video_id = Uuid::new_v4();
		repository.add_dislike(user_id, video_id).await.unwrap();

		let result = use_case.execute(user_id.to_string(), video_id.to_string()).await;

		assert!(result.is_ok());
		assert_eq!(repository.find_like_status(user_id, video_id).await.unwrap(), (false, false));
		assert_eq!(repository.dislike_count(video_id), 0);
	}

	#[tokio::test]
	async fn test_add_video_like_invalid_user_id() {
		let repository = InMemoryVideoReactionRepository::new();
		let use_case = AddVideoLike {
			reaction_repository: &repository,
		};

		let result = use_case
			.execute("invalid-user".to_string(), Uuid::new_v4().to_string())
			.await;

		assert!(result.is_err());
		assert!(repository.reactions.lock().unwrap().is_empty());
	}

	#[tokio::test]
	async fn test_video_reaction_use_cases_with_in_memory_repository_full_flow() {
		let repository = InMemoryVideoReactionRepository::new();
		let get_status = GetVideoReactionStatus {
			reaction_repository: &repository,
		};
		let add_like = AddVideoLike {
			reaction_repository: &repository,
		};
		let remove_like = RemoveVideoLike {
			reaction_repository: &repository,
		};
		let add_dislike = AddVideoDislike {
			reaction_repository: &repository,
		};
		let remove_dislike = RemoveVideoDislike {
			reaction_repository: &repository,
		};

		let user_id = Uuid::new_v4();
		let video_id = Uuid::new_v4();

		add_like
			.execute(user_id.to_string(), video_id.to_string())
			.await
			.unwrap();
		assert_eq!(
			get_status
				.execute(user_id.to_string(), video_id.to_string())
				.await
				.unwrap(),
			(true, false)
		);
		assert_eq!(repository.like_count(video_id), 1);
		assert_eq!(repository.dislike_count(video_id), 0);

		add_dislike
			.execute(user_id.to_string(), video_id.to_string())
			.await
			.unwrap();
		assert_eq!(
			get_status
				.execute(user_id.to_string(), video_id.to_string())
				.await
				.unwrap(),
			(false, true)
		);
		assert_eq!(repository.like_count(video_id), 0);
		assert_eq!(repository.dislike_count(video_id), 1);

		remove_like
			.execute(user_id.to_string(), video_id.to_string())
			.await
			.unwrap();
		assert_eq!(
			get_status
				.execute(user_id.to_string(), video_id.to_string())
				.await
				.unwrap(),
			(false, true)
		);
		assert_eq!(repository.like_count(video_id), 0);
		assert_eq!(repository.dislike_count(video_id), 1);

		remove_dislike
			.execute(user_id.to_string(), video_id.to_string())
			.await
			.unwrap();
		assert_eq!(
			get_status
				.execute(user_id.to_string(), video_id.to_string())
				.await
				.unwrap(),
			(false, false)
		);
		assert_eq!(repository.like_count(video_id), 0);
		assert_eq!(repository.dislike_count(video_id), 0);
	}

	#[tokio::test]
	async fn test_add_video_like_with_in_memory_repository_is_idempotent() {
		let repository = InMemoryVideoReactionRepository::new();
		let use_case = AddVideoLike {
			reaction_repository: &repository,
		};
		let user_id = Uuid::new_v4();
		let video_id = Uuid::new_v4();

		use_case
			.execute(user_id.to_string(), video_id.to_string())
			.await
			.unwrap();
		use_case
			.execute(user_id.to_string(), video_id.to_string())
			.await
			.unwrap();

		assert_eq!(repository.like_count(video_id), 1);
		assert_eq!(repository.dislike_count(video_id), 0);
		assert_eq!(repository.reactions.lock().unwrap().len(), 1);
	}
}

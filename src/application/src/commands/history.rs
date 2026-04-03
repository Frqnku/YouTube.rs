use crate::_helpers::parse_uuid;
use domain::video::VideoHistoryRepository;

pub struct CleanHistory<
	'a,
	R: VideoHistoryRepository,
> {
	pub video_repository: &'a R,
}

impl<'a, R> CleanHistory<'a, R>
where
	R: VideoHistoryRepository,
{
	pub async fn execute(&self, user_id: String) -> anyhow::Result<()> {
		let user_id = parse_uuid(&user_id, "user id")?;

		self
			.video_repository
			.clean_history_by_user_id(user_id)
			.await
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::_tests::repositories::{InMemoryVideoRepository, VideoHistoryEntry};
	use uuid::Uuid;

	#[tokio::test]
	async fn test_clean_history_success() {
		let repository = InMemoryVideoRepository::new();
		let command = CleanHistory {
			video_repository: &repository,
		};

		let user_id = Uuid::new_v4();
		let other_user_id = Uuid::new_v4();

		repository.history_entries.lock().unwrap().push(VideoHistoryEntry {
			user_id,
			video_id: Uuid::new_v4(),
			updated_at: chrono::Utc::now(),
		});
		repository.history_entries.lock().unwrap().push(VideoHistoryEntry {
			user_id: other_user_id,
			video_id: Uuid::new_v4(),
			updated_at: chrono::Utc::now(),
		});

		command.execute(user_id.to_string()).await.unwrap();

		let entries = repository.history_entries.lock().unwrap();
		assert_eq!(entries.len(), 1);
		assert_eq!(entries[0].user_id, other_user_id);
	}

	#[tokio::test]
	async fn test_clean_history_invalid_user_id() {
		let repository = InMemoryVideoRepository::new();
		let command = CleanHistory {
			video_repository: &repository,
		};

		let result = command.execute("invalid-user-id".to_string()).await;

		assert!(result.is_err());
	}
}

use domain::{
	_shared::DomainError,
	video::VideoViewRepository,
};
use uuid::Uuid;

const RECOUNT_AFTER_SECONDS: i64 = 5 * 60;

fn parse_uuid(id: &str) -> anyhow::Result<Uuid> {
	Uuid::parse_str(id).map_err(|_| DomainError::VideoNotFound.into())
}

pub struct RegisterVideoView<
	'a,
	R: VideoViewRepository,
> {
	pub view_repository: &'a R,
}

impl<'a, R> RegisterVideoView<'a, R>
where
	R: VideoViewRepository,
{
	pub async fn execute(
		&self,
		video_id: String,
		user_id: Option<String>,
		ip_address: Option<String>,
	) -> anyhow::Result<()> {
		let video_id = parse_uuid(&video_id)?;
		let user_id = user_id
			.map(|id| parse_uuid(&id))
			.transpose()?;
		let ip_address = ip_address
			.map(|ip| ip.trim().to_string())
			.filter(|ip| !ip.is_empty());

		self
			.view_repository
			.register_view(video_id, user_id, ip_address, RECOUNT_AFTER_SECONDS)
			.await
	}
}

pub struct UpdateWatchedSeconds<
	'a,
	R: VideoViewRepository,
> {
	pub view_repository: &'a R,
}

impl<'a, R> UpdateWatchedSeconds<'a, R>
where
	R: VideoViewRepository,
{
	pub async fn execute(&self, video_id: String, user_id: String, watched_seconds: u32) -> anyhow::Result<()> {
		let video_id = parse_uuid(&video_id)?;
		let user_id = parse_uuid(&user_id)?;

		self
			.view_repository
			.update_watched_seconds(video_id, user_id, watched_seconds)
			.await
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::_tests::repositories::InMemoryVideoViewRepository;

	#[tokio::test]
	async fn test_register_video_view_success() {
		let repository = InMemoryVideoViewRepository::new();
		let use_case = RegisterVideoView {
			view_repository: &repository,
		};

		let video_id = Uuid::new_v4();
		let user_id = Uuid::new_v4();

		let result = use_case
			.execute(
				video_id.to_string(),
				Some(user_id.to_string()),
				Some(" 127.0.0.1 ".to_string()),
			)
			.await;

		assert!(result.is_ok());

		let views = repository.views.lock().unwrap();
		assert_eq!(views.len(), 1);
		assert_eq!(views[0].video_id, video_id);
		assert_eq!(views[0].user_id, Some(user_id));
		assert_eq!(views[0].ip_address.as_deref(), Some("127.0.0.1"));
		assert_eq!(repository.view_count(video_id), 1);
	}

	#[tokio::test]
	async fn test_register_video_view_invalid_video_id() {
		let repository = InMemoryVideoViewRepository::new();
		let use_case = RegisterVideoView {
			view_repository: &repository,
		};

		let result = use_case
			.execute("invalid-uuid".to_string(), None, Some("1.1.1.1".to_string()))
			.await;

		assert!(result.is_err());
		assert!(repository.views.lock().unwrap().is_empty());
	}

	#[tokio::test]
	async fn test_register_video_view_with_in_memory_repository_does_not_recount_recent_same_ip() {
		let repository = InMemoryVideoViewRepository::new();
		let use_case = RegisterVideoView {
			view_repository: &repository,
		};
		let video_id = Uuid::new_v4();

		use_case
			.execute(video_id.to_string(), None, Some(" 127.0.0.1 ".to_string()))
			.await
			.unwrap();
		use_case
			.execute(video_id.to_string(), None, Some("127.0.0.1".to_string()))
			.await
			.unwrap();

		assert_eq!(repository.view_count(video_id), 1);
		assert_eq!(repository.views.lock().unwrap().len(), 1);
	}

	#[tokio::test]
	async fn test_register_video_view_with_in_memory_repository_promotes_anonymous_view_to_user() {
		let repository = InMemoryVideoViewRepository::new();
		let use_case = RegisterVideoView {
			view_repository: &repository,
		};
		let video_id = Uuid::new_v4();
		let user_id = Uuid::new_v4();

		use_case
			.execute(video_id.to_string(), None, Some("127.0.0.1".to_string()))
			.await
			.unwrap();
		use_case
			.execute(
				video_id.to_string(),
				Some(user_id.to_string()),
				Some("127.0.0.1".to_string()),
			)
			.await
			.unwrap();

		let views = repository.views.lock().unwrap();
		assert_eq!(views.len(), 1);
		assert_eq!(views[0].user_id, Some(user_id));
		assert_eq!(views[0].ip_address.as_deref(), Some("127.0.0.1"));
		assert_eq!(repository.view_count(video_id), 1);
	}
}

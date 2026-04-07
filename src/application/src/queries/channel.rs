use crate::dtos::ChannelDataDto;
use crate::_helpers::parse_uuid;
use domain::{
	channel::ChannelRepository,
};

pub struct GetChannelData<
	'a,
	R: ChannelRepository,
> {
	pub channel_repository: &'a R,
}

impl<'a, R> GetChannelData<'a, R>
where
	R: ChannelRepository,
{
	pub async fn execute(&self, channel_id: String) -> anyhow::Result<Option<ChannelDataDto>> {
		let channel_id = parse_uuid(&channel_id, "channel id")?;

		let channel = self
			.channel_repository
			.find_channel_by_id(channel_id)
			.await?;

		Ok(channel.map(ChannelDataDto::from))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::_tests::repositories::InMemoryChannelRepository;
	use domain::channel::entity::Channel;
	use uuid::Uuid;

	#[tokio::test]
	async fn get_channel_data_returns_dto_when_channel_exists() {
		let channel_id = Uuid::new_v4();
		let channel = Channel::new(
			channel_id,
			"My Channel".to_string(),
			None,
			None,
			Some("desc".to_string()),
			42,
			7,
		);
		let repo = InMemoryChannelRepository::new();
		*repo.channel.lock().unwrap() = Some(channel);
		let query = GetChannelData {
			channel_repository: &repo,
		};

		let result = query.execute(channel_id.to_string()).await.unwrap();

		let dto = result.expect("channel should exist");
		assert_eq!(dto.id, channel_id);
		assert_eq!(dto.name, "My Channel");
		assert_eq!(dto.subscriber_count, 42);
	}

	#[tokio::test]
	async fn get_channel_data_returns_none_when_channel_missing() {
		let channel_id = Uuid::new_v4();
		let repo = InMemoryChannelRepository::new();
		let query = GetChannelData {
			channel_repository: &repo,
		};

		let result = query.execute(channel_id.to_string()).await.unwrap();

		assert!(result.is_none());
		assert_eq!(repo.calls.lock().unwrap().len(), 1);
	}

	#[tokio::test]
	async fn get_channel_data_fails_on_invalid_channel_id() {
		let repo = InMemoryChannelRepository::new();
		let query = GetChannelData {
			channel_repository: &repo,
		};

		let result = query.execute("invalid".to_string()).await;

		assert!(result.is_err());
		assert!(repo.calls.lock().unwrap().is_empty());
	}
}

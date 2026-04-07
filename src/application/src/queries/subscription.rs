use crate::dtos::ChannelDto;
use crate::_helpers::parse_uuid;
use domain::{
	channel::SubscriptionRepository,
};

pub struct GetSubscriptionStatus<
	'a,
	R: SubscriptionRepository,
> {
	pub subscription_repository: &'a R,
}

impl<'a, R> GetSubscriptionStatus<'a, R>
where
	R: SubscriptionRepository,
{
	pub async fn execute(&self, subscriber_id: String, channel_id: String) -> anyhow::Result<bool> {
		let subscriber_id = parse_uuid(&subscriber_id, "subscriber id")?;
		let channel_id = parse_uuid(&channel_id, "channel id")?;

		self.subscription_repository
			.is_subscribed(subscriber_id, channel_id)
			.await
	}
}

pub struct ListSubscriptions<
	'a,
	R: SubscriptionRepository,
> {
	pub subscription_repository: &'a R,
}

impl<'a, R> ListSubscriptions<'a, R>
where
	R: SubscriptionRepository,
{
	pub async fn execute(&self, subscriber_id: String) -> anyhow::Result<Vec<ChannelDto>> {
		let subscriber_id = parse_uuid(&subscriber_id, "subscriber id")?;

		let subscriptions = self
			.subscription_repository
			.list_subscriptions(subscriber_id)
			.await?;

		Ok(subscriptions.into_iter().map(ChannelDto::from).collect())
	}
}

pub struct CountSubscribers<
	'a,
	R: SubscriptionRepository,
> {
	pub subscription_repository: &'a R,
}

impl<'a, R> CountSubscribers<'a, R>
where
	R: SubscriptionRepository,
{
	pub async fn execute(&self, channel_id: String) -> anyhow::Result<usize> {
		let channel_id = parse_uuid(&channel_id, "channel id")?;

		self.subscription_repository
			.count_subscribers(channel_id)
			.await
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::_tests::repositories::InMemorySubscriptionRepository;
	use domain::channel::entity::Channel;
	use uuid::Uuid;

	#[tokio::test]
	async fn get_subscription_status_calls_repository() {
		let repo = InMemorySubscriptionRepository::new();
		*repo.status_result.lock().unwrap() = true;
		let query = GetSubscriptionStatus {
			subscription_repository: &repo,
		};
		let subscriber_id = Uuid::new_v4();
		let channel_id = Uuid::new_v4();

		let result = query
			.execute(subscriber_id.to_string(), channel_id.to_string())
			.await
			.unwrap();

		assert!(result);
		assert_eq!(repo.status_calls.lock().unwrap()[0], (subscriber_id, channel_id));
	}

	#[tokio::test]
	async fn list_subscriptions_maps_channels_to_dto() {
		let repo = InMemorySubscriptionRepository::new();
		*repo.list_result.lock().unwrap() = vec![Channel::new(
			Uuid::new_v4(),
			"Subscribed channel".to_string(),
			None,
			None,
			None,
			0,
			0,
		)];
		let query = ListSubscriptions {
			subscription_repository: &repo,
		};
		let subscriber_id = Uuid::new_v4();

		let result = query.execute(subscriber_id.to_string()).await.unwrap();

		assert_eq!(result.len(), 1);
		assert_eq!(result[0].name, "Subscribed channel");
		assert_eq!(repo.list_calls.lock().unwrap()[0], subscriber_id);
	}

	#[tokio::test]
	async fn count_subscribers_calls_repository() {
		let repo = InMemorySubscriptionRepository::new();
		*repo.count_result.lock().unwrap() = 33;
		let query = CountSubscribers {
			subscription_repository: &repo,
		};
		let channel_id = Uuid::new_v4();

		let result = query.execute(channel_id.to_string()).await.unwrap();

		assert_eq!(result, 33);
		assert_eq!(repo.count_calls.lock().unwrap()[0], channel_id);
	}

	#[tokio::test]
	async fn get_subscription_status_fails_on_invalid_id() {
		let repo = InMemorySubscriptionRepository::new();
		let query = GetSubscriptionStatus {
			subscription_repository: &repo,
		};

		let result = query
			.execute("invalid".to_string(), Uuid::new_v4().to_string())
			.await;

		assert!(result.is_err());
		assert!(repo.status_calls.lock().unwrap().is_empty());
	}
}

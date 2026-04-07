use crate::_helpers::parse_uuid;
use domain::{
	channel::SubscriptionRepository,
};

pub struct Subscribe<
	'a,
	R: SubscriptionRepository,
> {
	pub subscription_repository: &'a R,
}

impl<'a, R> Subscribe<'a, R>
where
	R: SubscriptionRepository,
{
	pub async fn execute(&self, subscriber_id: String, channel_id: String) -> anyhow::Result<()> {
		let subscriber_id = parse_uuid(&subscriber_id, "subscriber id")?;
		let channel_id = parse_uuid(&channel_id, "channel id")?;

		self.subscription_repository
			.subscribe(subscriber_id, channel_id)
			.await
	}
}

pub struct Unsubscribe<
	'a,
	R: SubscriptionRepository,
> {
	pub subscription_repository: &'a R,
}

impl<'a, R> Unsubscribe<'a, R>
where
	R: SubscriptionRepository,
{
	pub async fn execute(&self, subscriber_id: String, channel_id: String) -> anyhow::Result<()> {
		let subscriber_id = parse_uuid(&subscriber_id, "subscriber id")?;
		let channel_id = parse_uuid(&channel_id, "channel id")?;

		self.subscription_repository
			.unsubscribe(subscriber_id, channel_id)
			.await
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::_tests::repositories::InMemorySubscriptionRepository;
	use uuid::Uuid;

	#[tokio::test]
	async fn subscribe_calls_repository_with_parsed_ids() {
		let repo = InMemorySubscriptionRepository::new();
		let command = Subscribe {
			subscription_repository: &repo,
		};

		let subscriber_id = Uuid::new_v4();
		let channel_id = Uuid::new_v4();

		command
			.execute(subscriber_id.to_string(), channel_id.to_string())
			.await
			.unwrap();

		let calls = repo.subscribe_calls.lock().unwrap();
		assert_eq!(calls.len(), 1);
		assert_eq!(calls[0], (subscriber_id, channel_id));
	}

	#[tokio::test]
	async fn subscribe_fails_on_invalid_subscriber_id() {
		let repo = InMemorySubscriptionRepository::new();
		let command = Subscribe {
			subscription_repository: &repo,
		};

		let result = command
			.execute("not-a-uuid".to_string(), Uuid::new_v4().to_string())
			.await;

		assert!(result.is_err());
		assert!(repo.subscribe_calls.lock().unwrap().is_empty());
	}

	#[tokio::test]
	async fn unsubscribe_calls_repository_with_parsed_ids() {
		let repo = InMemorySubscriptionRepository::new();
		let command = Unsubscribe {
			subscription_repository: &repo,
		};

		let subscriber_id = Uuid::new_v4();
		let channel_id = Uuid::new_v4();

		command
			.execute(subscriber_id.to_string(), channel_id.to_string())
			.await
			.unwrap();

		let calls = repo.unsubscribe_calls.lock().unwrap();
		assert_eq!(calls.len(), 1);
		assert_eq!(calls[0], (subscriber_id, channel_id));
	}

	#[tokio::test]
	async fn unsubscribe_fails_on_invalid_channel_id() {
		let repo = InMemorySubscriptionRepository::new();
		let command = Unsubscribe {
			subscription_repository: &repo,
		};

		let result = command
			.execute(Uuid::new_v4().to_string(), "not-a-uuid".to_string())
			.await;

		assert!(result.is_err());
		assert!(repo.unsubscribe_calls.lock().unwrap().is_empty());
	}
}

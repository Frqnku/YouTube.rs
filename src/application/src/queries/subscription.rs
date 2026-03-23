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

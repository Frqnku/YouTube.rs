use domain::{
	_shared::DomainError,
	channel::SubscriptionRepository,
};
use uuid::Uuid;

fn parse_uuid(id: &str, field_name: &str) -> anyhow::Result<Uuid> {
	Uuid::parse_str(id)
		.map_err(|_| DomainError::BadRequest(format!("Invalid {field_name}")))
		.map_err(Into::into)
}

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

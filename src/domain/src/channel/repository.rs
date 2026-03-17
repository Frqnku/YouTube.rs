use uuid::Uuid;

use crate::channel::entity::Channel;

#[async_trait::async_trait]
pub trait ChannelRepository {
    async fn find_channel_by_id(&self, channel_id: Uuid) -> anyhow::Result<Option<Channel>>;
}

#[async_trait::async_trait]
pub trait SubscriptionRepository {
    async fn subscribe(&self, subscriber_id: Uuid, channel_id: Uuid) -> anyhow::Result<()>;
    async fn unsubscribe(&self, subscriber_id: Uuid, channel_id: Uuid) -> anyhow::Result<()>;
    async fn is_subscribed(&self, subscriber_id: Uuid, channel_id: Uuid) -> anyhow::Result<bool>;
    async fn list_subscriptions(&self, subscriber_id: Uuid) -> anyhow::Result<Vec<Channel>>;
    async fn count_subscribers(&self, channel_id: Uuid) -> anyhow::Result<usize>;
}
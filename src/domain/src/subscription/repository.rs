use uuid::Uuid;

use crate::user::entity::User;

#[async_trait::async_trait]
pub trait SubscriptionRepository {
    async fn subscribe(&self, subscriber_id: Uuid, channel_id: Uuid) -> anyhow::Result<()>;
    async fn unsubscribe(&self, subscriber_id: Uuid, channel_id: Uuid) -> anyhow::Result<()>;
    async fn is_subscribed(&self, subscriber_id: Uuid, channel_id: Uuid) -> anyhow::Result<bool>;
    async fn list_subscriptions(&self, subscriber_id: Uuid) -> anyhow::Result<Vec<User>>;
    async fn count_subscribers(&self, channel_id: Uuid) -> anyhow::Result<usize>;
}
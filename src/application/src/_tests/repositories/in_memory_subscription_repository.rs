use std::sync::{Arc, Mutex};

use domain::channel::{SubscriptionRepository, entity::Channel};
use uuid::Uuid;

pub struct InMemorySubscriptionRepository {
    pub subscribe_calls: Arc<Mutex<Vec<(Uuid, Uuid)>>>,
    pub unsubscribe_calls: Arc<Mutex<Vec<(Uuid, Uuid)>>>,
    pub status_calls: Arc<Mutex<Vec<(Uuid, Uuid)>>>,
    pub list_calls: Arc<Mutex<Vec<Uuid>>>,
    pub count_calls: Arc<Mutex<Vec<Uuid>>>,
    pub status_result: Arc<Mutex<bool>>,
    pub list_result: Arc<Mutex<Vec<Channel>>>,
    pub count_result: Arc<Mutex<usize>>,
}

impl InMemorySubscriptionRepository {
    pub fn new() -> Self {
        Self {
            subscribe_calls: Arc::new(Mutex::new(Vec::new())),
            unsubscribe_calls: Arc::new(Mutex::new(Vec::new())),
            status_calls: Arc::new(Mutex::new(Vec::new())),
            list_calls: Arc::new(Mutex::new(Vec::new())),
            count_calls: Arc::new(Mutex::new(Vec::new())),
            status_result: Arc::new(Mutex::new(false)),
            list_result: Arc::new(Mutex::new(Vec::new())),
            count_result: Arc::new(Mutex::new(0)),
        }
    }
}

#[async_trait::async_trait]
impl SubscriptionRepository for InMemorySubscriptionRepository {
    async fn subscribe(&self, subscriber_id: Uuid, channel_id: Uuid) -> anyhow::Result<()> {
        self.subscribe_calls
            .lock()
            .unwrap()
            .push((subscriber_id, channel_id));
        Ok(())
    }

    async fn unsubscribe(&self, subscriber_id: Uuid, channel_id: Uuid) -> anyhow::Result<()> {
        self.unsubscribe_calls
            .lock()
            .unwrap()
            .push((subscriber_id, channel_id));
        Ok(())
    }

    async fn is_subscribed(&self, subscriber_id: Uuid, channel_id: Uuid) -> anyhow::Result<bool> {
        self.status_calls
            .lock()
            .unwrap()
            .push((subscriber_id, channel_id));
        Ok(*self.status_result.lock().unwrap())
    }

    async fn list_subscriptions(&self, subscriber_id: Uuid) -> anyhow::Result<Vec<Channel>> {
        self.list_calls.lock().unwrap().push(subscriber_id);
        Ok(self.list_result.lock().unwrap().clone())
    }

    async fn count_subscribers(&self, channel_id: Uuid) -> anyhow::Result<usize> {
        self.count_calls.lock().unwrap().push(channel_id);
        Ok(*self.count_result.lock().unwrap())
    }
}

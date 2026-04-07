use std::sync::{Arc, Mutex};

use domain::channel::{ChannelRepository, entity::Channel};
use uuid::Uuid;

pub struct InMemoryChannelRepository {
    pub calls: Arc<Mutex<Vec<Uuid>>>,
    pub channel: Arc<Mutex<Option<Channel>>>,
}

impl InMemoryChannelRepository {
    pub fn new() -> Self {
        Self {
            calls: Arc::new(Mutex::new(Vec::new())),
            channel: Arc::new(Mutex::new(None)),
        }
    }
}

#[async_trait::async_trait]
impl ChannelRepository for InMemoryChannelRepository {
    async fn find_channel_by_id(&self, channel_id: Uuid) -> anyhow::Result<Option<Channel>> {
        self.calls.lock().unwrap().push(channel_id);
        Ok(self.channel.lock().unwrap().clone())
    }
}

use std::sync::{Arc, Mutex};

use domain::comment::CommentLikeRepository;
use uuid::Uuid;

pub struct InMemoryCommentLikeRepository {
    pub add_calls: Arc<Mutex<Vec<(Uuid, Uuid)>>>,
    pub remove_calls: Arc<Mutex<Vec<(Uuid, Uuid)>>>,
    pub liked: Arc<Mutex<Vec<(Uuid, Uuid)>>>,
}

impl InMemoryCommentLikeRepository {
    pub fn new() -> Self {
        Self {
            add_calls: Arc::new(Mutex::new(Vec::new())),
            remove_calls: Arc::new(Mutex::new(Vec::new())),
            liked: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait::async_trait]
impl CommentLikeRepository for InMemoryCommentLikeRepository {
    async fn is_liked_by_user(&self, comment_id: Uuid, user_id: Uuid) -> anyhow::Result<bool> {
        Ok(self
            .liked
            .lock()
            .unwrap()
            .iter()
            .any(|(c, u)| *c == comment_id && *u == user_id))
    }

    async fn add_like(&self, comment_id: Uuid, user_id: Uuid) -> anyhow::Result<()> {
        self.add_calls.lock().unwrap().push((comment_id, user_id));
        let mut liked = self.liked.lock().unwrap();
        if !liked.iter().any(|(c, u)| *c == comment_id && *u == user_id) {
            liked.push((comment_id, user_id));
        }
        Ok(())
    }

    async fn remove_like(&self, comment_id: Uuid, user_id: Uuid) -> anyhow::Result<()> {
        self.remove_calls.lock().unwrap().push((comment_id, user_id));
        self.liked
            .lock()
            .unwrap()
            .retain(|(c, u)| !(*c == comment_id && *u == user_id));
        Ok(())
    }
}

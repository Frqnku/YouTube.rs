use std::sync::{Arc, Mutex};

use domain::comment::{CommentPage, CommentPageRequest, CommentRepository, entity::{Comment, CommentAuthor}};
use uuid::Uuid;

fn sample_comment(comment_id: Uuid, user_id: Uuid, content: String) -> Comment {
    Comment::new(
        comment_id,
        Uuid::new_v4(),
        CommentAuthor::new(user_id, "Alice".to_string(), None),
        None,
        content,
        0,
        0,
        Some(false),
    )
}

pub struct InMemoryCommentRepository {
    pub saved: Arc<Mutex<Vec<Comment>>>,
    pub updated: Arc<Mutex<Vec<(Uuid, Uuid, String)>>>,
    pub deleted: Arc<Mutex<Vec<(Uuid, Uuid)>>>,
    pub find_calls: Arc<Mutex<Vec<(Uuid, Option<Uuid>)>>>,
    pub list_calls: Arc<Mutex<Vec<(Uuid, Option<Uuid>, CommentPageRequest, Option<Uuid>)>>>,
    pub replies_calls: Arc<Mutex<Vec<(Uuid, CommentPageRequest, Option<Uuid>)>>>,
    pub count_calls: Arc<Mutex<Vec<Uuid>>>,
    pub find_result: Arc<Mutex<Option<Comment>>>,
    pub list_result: Arc<Mutex<CommentPage>>,
    pub replies_result: Arc<Mutex<CommentPage>>,
    pub count_result: Arc<Mutex<i64>>,
}

impl InMemoryCommentRepository {
    pub fn new() -> Self {
        Self {
            saved: Arc::new(Mutex::new(Vec::new())),
            updated: Arc::new(Mutex::new(Vec::new())),
            deleted: Arc::new(Mutex::new(Vec::new())),
            find_calls: Arc::new(Mutex::new(Vec::new())),
            list_calls: Arc::new(Mutex::new(Vec::new())),
            replies_calls: Arc::new(Mutex::new(Vec::new())),
            count_calls: Arc::new(Mutex::new(Vec::new())),
            find_result: Arc::new(Mutex::new(None)),
            list_result: Arc::new(Mutex::new(CommentPage::new(Vec::new(), None, false))),
            replies_result: Arc::new(Mutex::new(CommentPage::new(Vec::new(), None, false))),
            count_result: Arc::new(Mutex::new(0)),
        }
    }
}

#[async_trait::async_trait]
impl CommentRepository for InMemoryCommentRepository {
    async fn find_by_id(&self, id: Uuid, viewer_user_id: Option<Uuid>) -> anyhow::Result<Option<Comment>> {
        self.find_calls.lock().unwrap().push((id, viewer_user_id));
        Ok(self.find_result.lock().unwrap().clone())
    }

    async fn list_by_video_id(
        &self,
        video_id: Uuid,
        parent_id: Option<Uuid>,
        page: CommentPageRequest,
        viewer_user_id: Option<Uuid>,
    ) -> anyhow::Result<CommentPage> {
        self.list_calls
            .lock()
            .unwrap()
            .push((video_id, parent_id, page, viewer_user_id));
        Ok(self.list_result.lock().unwrap().clone())
    }

    async fn list_replies(
        &self,
        parent_id: Uuid,
        page: CommentPageRequest,
        viewer_user_id: Option<Uuid>,
    ) -> anyhow::Result<CommentPage> {
        self.replies_calls
            .lock()
            .unwrap()
            .push((parent_id, page, viewer_user_id));
        Ok(self.replies_result.lock().unwrap().clone())
    }

    async fn count_by_video_id(&self, video_id: Uuid) -> anyhow::Result<i64> {
        self.count_calls.lock().unwrap().push(video_id);
        Ok(*self.count_result.lock().unwrap())
    }

    async fn count_replies(&self, _parent_id: Uuid) -> anyhow::Result<i64> {
        Ok(0)
    }

    async fn save(&self, comment: &Comment) -> anyhow::Result<Comment> {
        self.saved.lock().unwrap().push(comment.clone());
        Ok(comment.clone())
    }

    async fn update_content(&self, comment_id: Uuid, user_id: Uuid, content: String) -> anyhow::Result<Comment> {
        self.updated
            .lock()
            .unwrap()
            .push((comment_id, user_id, content.clone()));
        Ok(sample_comment(comment_id, user_id, content))
    }

    async fn delete(&self, comment_id: Uuid, user_id: Uuid) -> anyhow::Result<()> {
        self.deleted.lock().unwrap().push((comment_id, user_id));
        Ok(())
    }
}

use uuid::Uuid;

use crate::{_shared::value_objects::Url, user::value_objects::Email};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: Email,
    pub profile_picture: Option<Url>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl User {
    pub fn new(id: Uuid, name: String, email: Email, profile_picture: Option<Url>) -> Self {
        Self { id, name, email, profile_picture, created_at: chrono::Utc::now() }
    }
}
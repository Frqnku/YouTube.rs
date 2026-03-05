use domain::user::value_objects::{Email, OAuthProvider};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use super::_utils::{non_empty_string, valid_email};

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CurrentUserDto {
    pub id: Uuid,
    #[validate(custom(function = "non_empty_string"))]
    pub name: String,
    pub profile_picture: Option<String>,
}

impl CurrentUserDto {
    pub fn new(id: Uuid, name: String, profile_picture: Option<String>) -> Self {
        Self { id, name, profile_picture }
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct OAuthUserDto {
    pub provider: OAuthProvider,
    #[validate(custom(function = "non_empty_string"))]
    pub provider_user_id: String,
    #[validate(custom(function = "valid_email"))]
    pub email: Email,
    #[validate(custom(function = "non_empty_string"))]
    pub name: String,
    pub picture: Option<String>,
}

impl OAuthUserDto {
    pub fn new(
        provider: OAuthProvider,
        provider_user_id: String,
        email: Email,
        name: String,
        picture: Option<String>,
    ) -> Self {
        Self { provider, provider_user_id, email, name, picture }
    }
}
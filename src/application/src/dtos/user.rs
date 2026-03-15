use domain::{_shared::value_objects::Url, user::value_objects::{Email, OAuthProvider}};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::_utils::{non_empty_string, valid_email};

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CurrentUserDto {
    pub id: Uuid,
    #[validate(custom(function = "non_empty_string"))]
    pub name: String,
    pub profile_picture: Option<Url>,
}

impl CurrentUserDto {
    pub fn new(id: Uuid, name: String, profile_picture: Option<Url>) -> Self {
        Self { id, name, profile_picture }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct OAuthUserDto {
    pub provider: OAuthProvider,
    #[validate(custom(function = "non_empty_string"))]
    pub provider_user_id: String,
    #[validate(custom(function = "valid_email"))]
    pub email: Email,
    #[validate(custom(function = "non_empty_string"))]
    pub name: String,
    pub picture: Option<Url>,
}

impl OAuthUserDto {
    pub fn new(
        provider: OAuthProvider,
        provider_user_id: String,
        email: Email,
        name: String,
        picture: Option<Url>,
    ) -> Self {
        Self { provider, provider_user_id, email, name, picture }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_user_dto_new_and_validate() {
        let dto = CurrentUserDto::new(
            Uuid::new_v4(),
            "Alice".to_string(),
            Some(Url::try_from("https://example.com/avatar.jpg").unwrap()),
        );

        assert!(dto.validate().is_ok());
        assert_eq!(dto.name, "Alice");
    }

    #[test]
    fn test_current_user_dto_validate_rejects_blank_name() {
        let dto = CurrentUserDto::new(Uuid::new_v4(), "   ".to_string(), None);
        assert!(dto.validate().is_err());
    }

    #[test]
    fn test_oauth_user_dto_new_and_validate() {
        let dto = OAuthUserDto::new(
            OAuthProvider::Google,
            "google-123".to_string(),
            Email::try_from("user@example.com").unwrap(),
            "Alice".to_string(),
            Some(Url::try_from("https://example.com/picture.jpg").unwrap()),
        );

        assert!(dto.validate().is_ok());
        assert_eq!(dto.provider, OAuthProvider::Google);
        assert_eq!(dto.provider_user_id, "google-123");
        assert_eq!(dto.name, "Alice");
    }

    #[test]
    fn test_oauth_user_dto_validate_rejects_blank_fields() {
        let dto = OAuthUserDto::new(
            OAuthProvider::Google,
            "   ".to_string(),
            Email::try_from("user@example.com").unwrap(),
            "   ".to_string(),
            None,
        );

        let errors = dto.validate().unwrap_err();
        assert!(errors.field_errors().contains_key("provider_user_id"));
        assert!(errors.field_errors().contains_key("name"));
    }

    #[test]
    fn test_oauth_user_dto_validate_rejects_invalid_email_after_deserialize() {
        let dto: OAuthUserDto = serde_json::from_value(serde_json::json!({
            "provider": "Google",
            "provider_user_id": "google-123",
            "email": "not-an-email",
            "name": "Alice",
            "picture": null
        }))
        .unwrap();

        let errors = dto.validate().unwrap_err();
        assert!(errors.field_errors().contains_key("email"));
    }
}
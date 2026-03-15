use domain::user::value_objects::Email;
use validator::{ValidateEmail, ValidateUrl, ValidationError};

pub fn non_empty_string(s: &str) -> Result<(), ValidationError> {
    if s.trim().is_empty() {
        let mut err = ValidationError::new("empty_string");
        err.message = Some("This field cannot be empty.".into());
        return Err(err);
    }
    Ok(())
}

pub fn valid_url(url: &str) -> Result<(), ValidationError> {
    if !url.to_string().validate_url() {
        let mut err = ValidationError::new("invalid_url");
        err.message = Some("This field must be a valid URL.".into());
        return Err(err);
    }
    Ok(())
}

pub fn valid_email(email: &Email) -> Result<(), ValidationError> {
    if !email.to_string().validate_email() {
        let mut err = ValidationError::new("invalid_email");
        err.message = Some("Email must be a valid email address.".into());
        return Err(err);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_empty_string() {
        assert!(non_empty_string("hello").is_ok());
        assert!(non_empty_string("   ").is_err());
    }

    #[test]
    fn test_valid_url() {
        assert!(valid_url("https://example.com/video").is_ok());
        assert!(valid_url("not-a-url").is_err());
    }

    #[test]
    fn test_valid_email() {
        let email = Email::try_from("user@example.com").unwrap();
        assert!(super::valid_email(&email).is_ok());

        // Email derives Deserialize and can be deserialized from raw strings
        // without domain validation, so this checks the DTO validation guard.
        let invalid: Email = serde_json::from_str("\"not-an-email\"").unwrap();
        assert!(super::valid_email(&invalid).is_err());
    }
}
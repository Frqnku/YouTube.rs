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
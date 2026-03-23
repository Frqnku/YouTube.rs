use serde::{Deserialize, Serialize};
use std::fmt;
use regex::Regex;

use crate::_shared::DomainError;

/// Email value object with validation.
///
/// Enforces email format validation at the type level.
/// Matches the database constraint: ^[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}$
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    /// Creates a new Email without validation (internal use only).
    fn new_unchecked(email: String) -> Self {
        Email(email)
    }

    /// Returns the email as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes self and returns the inner string.
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl TryFrom<String> for Email {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 255 {
            return Err(DomainError::BadRequest("Email too long (max 255 characters)".to_string()));
        }

        let regex = Regex::new(r"(?i)^[a-z0-9]+([._+-]?[a-z0-9]+)*@([a-z0-9]+(-[a-z0-9]+)*\.)+[a-z]{2,}$")
            .map_err(|_| DomainError::BadRequest("Invalid email".to_string()))?;
        if regex.is_match(&value) {
            Ok(Email::new_unchecked(value))
        } else {
            Err(DomainError::BadRequest("Invalid email format".to_string()))
        }
    }
}

impl TryFrom<&str> for Email {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Email::try_from(value.to_string())
    }
}

impl TryFrom<&String> for Email {
    type Error = DomainError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Email::try_from(value.to_string())
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        let valid_emails = [
            "user@example.com",
            "user.name+tag+sorting@example.com",
            "user_name@example.co.uk",
            "user-name@example.org",
            "user123@example.io",
        ];

        for email_str in valid_emails.iter() {
            let email = Email::try_from(email_str.to_string()).unwrap();
            assert_eq!(email.as_str(), *email_str, "Email '{}' should be valid", email_str);
        }
    }

    #[test]
    fn test_invalid_email_format() {
        let invalid_emails = [
            "plainaddress",
            "@missingusername.com",
            "username@.com",
            "username@com",
            "username@.com.",
            "username@-example.com",
            "username@example..com",
        ];

        for email_str in invalid_emails.iter() {
            assert!(Email::try_from(email_str.to_string()).is_err(), "Email '{}' should be invalid", email_str);
        }
    }

    #[test]
    fn test_email_too_long() {
        let long_email = format!("{}@example.com", "a".repeat(250));
        assert!(Email::try_from(&long_email).is_err(), "Email '{}' should be invalid", long_email);
    }
}

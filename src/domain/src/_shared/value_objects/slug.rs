use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::fmt;
use crate::_shared::DomainError;

/// Slug value object with validation.
///
/// A URL-friendly identifier composed of lowercase letters, numbers, and hyphens.
/// Matches the database constraint: ^[a-z0-9]+(?:-[a-z0-9]+)*$
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Slug(String);

impl Slug {
    /// Creates a new Slug without validation (internal use only).
    fn new_unchecked(slug: String) -> Self {
        Slug(slug)
    }

    /// Returns the slug as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes self and returns the inner string.
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Generates a slug from a free-form string.
    /// Converts to lowercase and replaces spaces/underscores with hyphens.
    pub fn from_text(text: &str) -> Result<Self, DomainError> {
        if text.is_empty() {
            return Err(DomainError::BadRequest("Slug is empty".to_string()));
        }

        let slug = text
            .to_lowercase()
            .trim()
            .chars()
            .map(|c| {
                if c.is_alphanumeric() {
                    c
                } else if c == ' ' || c == '_' || c == '-' {
                    '-'
                } else {
                    // Skip other characters
                    ' '
                }
            })
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join("-")
            // Remove consecutive hyphens
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()
            .join("-");

        Slug::try_from(slug)
    }
}

impl TryFrom<String> for Slug {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(DomainError::BadRequest("Slug is empty".to_string()));
        }

        if value.len() > 100 {
            return Err(DomainError::BadRequest("Slug too long (max 100 characters)".to_string()));
        }

        // Validate format: ^[a-z0-9]+(?:-[a-z0-9]+)*$
        if value
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
            && !value.starts_with('-')
            && !value.ends_with('-')
            && !value.contains("--")
        {
            Ok(Slug::new_unchecked(value + Uuid::new_v4().to_string().as_str()))
        } else {
            Err(DomainError::BadRequest(
                "Invalid slug format. Must contain only lowercase letters, numbers, and hyphens"
                    .to_string(),
            ))
        }
    }
}

impl TryFrom<&str> for Slug {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Slug::try_from(value.to_string())
    }
}

impl fmt::Display for Slug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Slug {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::_shared::DomainError;

    #[test]
    fn test_valid_slug() {
        let slug = Slug::try_from("john-doe".to_string()).unwrap();
        assert!(slug.as_str().starts_with("john-doe"));
        // Verify UUID suffix is present
        assert!(slug.as_str().len() > "john-doe".len());
    }

    #[test]
    fn test_invalid_slug_uppercase() {
        assert!(Slug::try_from("John-Doe".to_string()).is_err());
    }

    #[test]
    fn test_invalid_slug_leading_hyphen() {
        assert!(Slug::try_from("-john-doe".to_string()).is_err());
    }

    #[test]
    fn test_invalid_slug_consecutive_hyphens() {
        assert!(Slug::try_from("john--doe".to_string()).is_err());
    }

    #[test]
    fn test_slug_from_text() {
        let slug = Slug::from_text("John Doe").unwrap();
        assert!(slug.as_str().starts_with("john-doe"));
        assert!(slug.as_str().len() > "john-doe".len());

        let slug = Slug::from_text("Jane_Smith").unwrap();
        assert!(slug.as_str().starts_with("jane-smith"));
        assert!(slug.as_str().len() > "jane-smith".len());
    }

    #[test]
    fn test_slug_too_long() {
        let long_slug = "a".repeat(101);
        assert!(matches!(
            Slug::try_from(long_slug),
            Err(DomainError::BadRequest(msg)) if msg.contains("too long")
        ));
    }

    #[test]
    fn test_slug_empty() {
        assert!(matches!(
            Slug::try_from("".to_string()),
            Err(DomainError::BadRequest(msg)) if msg.contains("empty")
        ));
    }
}

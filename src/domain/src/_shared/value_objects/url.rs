use serde::{Deserialize, Serialize};
use std::fmt;
use crate::_shared::DomainError;

/// URL value object with validation.
///
/// Supports HTTP(S) URLs and absolute relative paths (starting with '/'),
/// and enforces a safe max length.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Url(String);

impl Url {
	/// Creates a new Url without validation (internal use only).
	fn new_unchecked(value: String) -> Self {
		Self(value)
	}

	/// Returns the URL as a string slice.
	pub fn as_str(&self) -> &str {
		&self.0
	}

	/// Consumes self and returns the inner string.
	pub fn into_inner(self) -> String {
		self.0
	}

	/// Returns true when the URL uses HTTPS.
	pub fn is_https(&self) -> bool {
		self.0.starts_with("https://")
	}

	fn is_absolute_relative_path(value: &str) -> bool {
		value.starts_with('/') && !value.contains(char::is_whitespace)
	}

	fn normalize_scheme(input: &str) -> String {
		if input.len() >= 7 && input[..7].eq_ignore_ascii_case("http://") {
			format!("http://{}", &input[7..])
		} else if input.len() >= 8 && input[..8].eq_ignore_ascii_case("https://") {
			format!("https://{}", &input[8..])
		} else {
			input.to_string()
		}
	}

	fn is_valid_format(value: &str) -> bool {
		// Basic URL guardrails without pulling extra parsing dependencies.
		if value.contains(char::is_whitespace) {
			return false;
		}

		let rest = if let Some(stripped) = value.strip_prefix("http://") {
			stripped
		} else if let Some(stripped) = value.strip_prefix("https://") {
			stripped
		} else {
			return false;
		};

		if rest.is_empty() {
			return false;
		}

		let host_end = rest.find(['/', '?', '#']).unwrap_or(rest.len());
		let host_port = &rest[..host_end];
		if host_port.is_empty() {
			return false;
		}

		let (host, port) = match host_port.rsplit_once(':') {
			Some((h, p)) if !h.is_empty() && p.chars().all(|c| c.is_ascii_digit()) => (h, Some(p)),
			Some((_, _)) => return false,
			None => (host_port, None),
		};

		if let Some(p) = port {
			if p.len() < 2 || p.len() > 5 {
				return false;
			}
		}

		host.chars()
			.all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-'))
	}
}

impl TryFrom<String> for Url {
	type Error = DomainError;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		let trimmed = value.trim();
		if trimmed.is_empty() {
			return Err(DomainError::BadRequest("Url is empty".to_string()));
		}

		if trimmed.len() > 2048 {
			return Err(DomainError::BadRequest(
				"Url too long (max 2048 characters)".to_string(),
			));
		}

		let normalized = Url::normalize_scheme(trimmed);
		if Url::is_absolute_relative_path(&normalized) {
			return Ok(Url::new_unchecked(normalized));
		}

		if !(normalized.starts_with("http://") || normalized.starts_with("https://")) {
			return Err(DomainError::BadRequest(
				"Unsupported URL format. Use http(s) URL or absolute path starting with '/'"
					.to_string(),
			));
		}

		if !Url::is_valid_format(&normalized) {
			return Err(DomainError::BadRequest("Invalid URL format".to_string()));
		}

		Ok(Url::new_unchecked(normalized))
	}
}

impl TryFrom<&str> for Url {
	type Error = DomainError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		Url::try_from(value.to_string())
	}
}

impl TryFrom<&String> for Url {
    type Error = DomainError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Url::try_from(value.to_string())
    }
}

impl fmt::Display for Url {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl AsRef<str> for Url {
	fn as_ref(&self) -> &str {
		&self.0
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::_shared::DomainError;

	#[test]
	fn test_valid_http_url() {
		let url = Url::try_from("http://example.com".to_string()).unwrap();
		assert_eq!(url.as_str(), "http://example.com");
		assert!(!url.is_https());
	}

	#[test]
	fn test_valid_https_url_with_path_query_and_fragment() {
		let url = Url::try_from("https://example.com/videos/abc123?q=rust#top").unwrap();
		assert_eq!(url.as_str(), "https://example.com/videos/abc123?q=rust#top");
		assert!(url.is_https());
	}

	#[test]
	fn test_valid_url_with_port() {
		let url = Url::try_from("https://localhost:3000/watch?v=42").unwrap();
		assert_eq!(url.as_str(), "https://localhost:3000/watch?v=42");
	}

	#[test]
	fn test_scheme_is_normalized_to_lowercase() {
		let url = Url::try_from("HTTPS://example.com/video".to_string()).unwrap();
		assert_eq!(url.as_str(), "https://example.com/video");
	}

	#[test]
	fn test_try_from_str_works() {
		let url = Url::try_from("https://example.com").unwrap();
		assert_eq!(url.as_str(), "https://example.com");
	}

	#[test]
	fn test_reject_empty_url() {
		assert!(matches!(
			Url::try_from("".to_string()),
			Err(DomainError::BadRequest(msg)) if msg.contains("empty")
		));
		assert!(matches!(
			Url::try_from("   ".to_string()),
			Err(DomainError::BadRequest(msg)) if msg.contains("empty")
		));
	}

	#[test]
	fn test_reject_unsupported_scheme() {
		assert!(matches!(
			Url::try_from("ftp://example.com".to_string()),
			Err(DomainError::BadRequest(msg)) if msg.contains("Unsupported URL format")
		));
		assert!(matches!(
			Url::try_from("example.com/video".to_string()),
			Err(DomainError::BadRequest(msg)) if msg.contains("Unsupported URL format")
		));
	}

	#[test]
	fn test_accept_absolute_relative_path() {
		let url = Url::try_from("/videos/rickroll.mp4").unwrap();
		assert_eq!(url.as_str(), "/videos/rickroll.mp4");
		assert!(!url.is_https());
	}

	#[test]
	fn test_reject_whitespace_in_url() {
		assert!(matches!(
			Url::try_from("https://example .com".to_string()),
			Err(DomainError::BadRequest(msg)) if msg.contains("Invalid URL format")
		));
	}

	#[test]
	fn test_reject_invalid_port() {
		assert!(matches!(
			Url::try_from("https://example.com:abc".to_string()),
			Err(DomainError::BadRequest(msg)) if msg.contains("Invalid URL format")
		));
		assert!(matches!(
			Url::try_from("https://example.com:1".to_string()),
			Err(DomainError::BadRequest(msg)) if msg.contains("Invalid URL format")
		));
	}

	#[test]
	fn test_reject_too_long_url() {
		let long = format!("https://example.com/{}", "a".repeat(2048));
		assert!(matches!(
			Url::try_from(long),
			Err(DomainError::BadRequest(msg)) if msg.contains("too long")
		));
	}

	#[test]
	fn test_as_ref_display_and_into_inner() {
		let url = Url::try_from("https://example.com/profile?id=42").unwrap();
		assert_eq!(url.as_ref(), "https://example.com/profile?id=42");
		assert_eq!(url.to_string(), "https://example.com/profile?id=42");

		let owned = url.into_inner();
		assert_eq!(owned, "https://example.com/profile?id=42");
	}
}

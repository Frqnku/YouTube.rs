use std::collections::HashMap;

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use axum::http::header::InvalidHeaderValue;
#[cfg(feature = "ssr")]
use domain::_shared::DomainError;
#[cfg(feature = "ssr")]
use validator::Validate;

use crate::api::_errors::AppServerErrorEncoding;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppServerError {
    pub code: String,
    pub message: String,
    pub fields: Option<HashMap<String, Vec<String>>>,
}

impl AppServerError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            fields: None,
        }
    }

    pub fn with_fields(mut self, fields: HashMap<String, Vec<String>>) -> Self {
        self.fields = Some(fields);
        self
    }
}

#[cfg(feature = "ssr")]
impl From<DomainError> for AppServerError {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::InvalidCredentials => AppServerError::new("invalid_credentials", err.to_string()),
            DomainError::InvalidToken => AppServerError::new("invalid_token", err.to_string()),
            DomainError::Unauthorized => AppServerError::new("unauthorized", err.to_string()),
            DomainError::BadRequest(_) => AppServerError::new("bad_request", err.to_string()),
            DomainError::DatabaseError(_) => AppServerError::new("database_error", err.to_string()),
            DomainError::Unexpected(_) => AppServerError::new("unexpected_error", err.to_string()),
        }
    }
}

impl From<anyhow::Error> for AppServerError {
    fn from(err: anyhow::Error) -> Self {
        AppServerError::new("unexpected_error", err.to_string())
    }
}

#[cfg(feature = "ssr")]
impl From<InvalidHeaderValue> for AppServerError {
    fn from(err: InvalidHeaderValue) -> Self {
        AppServerError::new("invalid_header_value", err.to_string())
    }
}

impl FromServerFnError for AppServerError {
    type Encoder = AppServerErrorEncoding;

    fn from_server_fn_error(err: ServerFnErrorErr) -> Self {
        AppServerError::new("server_function_error".to_string(), err.to_string())
    }
}

pub trait ValidateExt {
    fn validate_or_error(&self) -> Result<(), AppServerError>;
}

#[cfg(feature = "ssr")]
fn validation_errors_to_fields(errors: &validator::ValidationErrors) -> HashMap<String, Vec<String>> {
    errors
        .field_errors()
        .iter()
        .map(|(field, field_errors)| {
            let messages = field_errors
                .iter()
                .map(|e| {
                    e.message
                        .as_ref()
                        .map(ToString::to_string)
                        .unwrap_or_else(|| "Invalid value".to_string())
                })
                .collect();

            (field.to_string(), messages)
        })
        .collect()
}

#[cfg(feature = "ssr")]
impl<T: Validate> ValidateExt for T {
    fn validate_or_error(&self) -> Result<(), AppServerError> {
        self.validate().map_err(|e| {
            let fields = validation_errors_to_fields(&e);
            AppServerError::new("validation_error", "Invalid field(s)")
                .with_fields(fields)
        })
    }
}

pub trait OptionExt<T> {
    fn require_context(self, msg: impl Into<String>) -> Result<T, AppServerError>;
}

#[cfg(feature = "ssr")]
impl<T> OptionExt<T> for Option<T> {
    fn require_context(self, msg: impl Into<String>) -> Result<T, AppServerError> {
        self.ok_or_else(|| AppServerError::from(
            DomainError::Unexpected(anyhow::anyhow!(msg.into())),
        ))
    }
}

pub trait ResultExt<T, E> {
    fn map_app_error(self, msg: impl Into<String>) -> Result<T, AppServerError>;
}

#[cfg(feature = "ssr")]
impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn map_app_error(self, msg: impl Into<String>) -> Result<T, AppServerError> {
        self.or_else(|_| Err(AppServerError::from(
            DomainError::Unexpected(anyhow::anyhow!(msg.into())),
        )))
    }
}
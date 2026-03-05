use bytes::Bytes;
use leptos::server_fn::{ContentType, Decodes, Encodes, Format, FormatType};

use crate::api::_errors::AppServerError;

pub struct AppServerErrorEncoding;

impl ContentType for AppServerErrorEncoding {
    const CONTENT_TYPE: &'static str = "application/json";
}

impl FormatType for AppServerErrorEncoding {
    const FORMAT_TYPE: Format = Format::Text;
}

impl Encodes<AppServerError> for AppServerErrorEncoding {
    type Error = std::fmt::Error;

    fn encode(err: &AppServerError) -> Result<Bytes, Self::Error> {
        serde_json::to_vec(err)
            .map(Bytes::from)
            .map_err(|_| std::fmt::Error)
    }
}

impl Decodes<AppServerError> for AppServerErrorEncoding {
    type Error = String;

    fn decode(bytes: Bytes) -> Result<AppServerError, Self::Error> {
        serde_json::from_slice::<AppServerError>(&bytes)
            .map_err(|e| e.to_string())
    }
}

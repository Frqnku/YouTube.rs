#[cfg(feature = "ssr")]
use axum::http::header::{SET_COOKIE, HeaderValue};
#[cfg(feature = "ssr")]
use leptos_axum::ResponseOptions;

pub enum CookieOptions {
    HttpOnly,
    Secure,
    SameSiteLax,
    SameSiteStrict,
    SameSiteNone,
    Path(String),
    MaxAge(i64),
    Expires(String),
}

pub struct Cookie {
    pub name: String,
    pub value: String,
    pub options: Vec<CookieOptions>,
}

impl Cookie {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
            options: vec![],
        }
    }

    pub fn with_options(mut self, options: Vec<CookieOptions>) -> Self {
        self.options = options;
        self
    }

    #[cfg(feature = "ssr")]
    pub fn into_response_headers(&self, res: &ResponseOptions) -> () {
        let cookie = build_cookie(&self.name, &self.value, &self.options);
        if let Ok(header_value) = HeaderValue::from_str(&cookie) {
            res.insert_header(SET_COOKIE, header_value);
        }
    }
}

fn build_cookie(name: &str, value: &str, options: &[CookieOptions]) -> String {
    let mut cookie = format!("{}={}", name, value);
    for option in options {
        match option {
            CookieOptions::HttpOnly => cookie.push_str("; HttpOnly"),
            CookieOptions::Secure => cookie.push_str("; Secure"),
            CookieOptions::SameSiteLax => cookie.push_str("; SameSite=Lax"),
            CookieOptions::SameSiteStrict => cookie.push_str("; SameSite=Strict"),
            CookieOptions::SameSiteNone => cookie.push_str("; SameSite=None"),
            CookieOptions::Path(path) => cookie.push_str(&format!("; Path={}", path)),
            CookieOptions::MaxAge(seconds) => cookie.push_str(&format!("; Max-Age={}", seconds)),
            CookieOptions::Expires(date) => cookie.push_str(&format!("; Expires={}", date)),
        }
    }
    cookie
}
use application::{dtos::CurrentUserDto, services::TokenService};
use uuid::Uuid;

pub struct JwtService {
    secret: String,
}

impl JwtService {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}

impl TokenService for JwtService {
    fn generate_token(&self, user_id: &str, user_name: &str, user_avatar: Option<String>) -> anyhow::Result<String> {
        let claims = serde_json::json!({ "sub": user_id, "name": user_name, "avatar": user_avatar, "exp": (chrono::Utc::now() + chrono::Duration::hours(48)).timestamp() });
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(self.secret.as_bytes()),
        )?;
        Ok(token)
    }

    fn verify_token(&self, token: &str) -> anyhow::Result<CurrentUserDto> {
        let decoded = jsonwebtoken::decode::<serde_json::Value>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(self.secret.as_bytes()),
            &jsonwebtoken::Validation::default(),
        )?;

        // Check expiration
        let exp = decoded.claims["exp"].as_i64().ok_or_else(|| anyhow::anyhow!("Invalid token: missing exp"))?;
        let now = chrono::Utc::now().timestamp();
        if now > exp {
            return Err(anyhow::anyhow!("Token has expired"));
        }

        Ok(CurrentUserDto::new(
            decoded.claims["sub"]
                .as_str()
                .and_then(|s| Uuid::parse_str(s).ok())
                .ok_or_else(|| anyhow::anyhow!("Invalid token: missing sub"))?,
            decoded.claims["name"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("Invalid token: missing name"))?
                .to_string(),
            decoded.claims["avatar"]
                .as_str()
                .map(|s| s.to_string()),
        ))
    }
}
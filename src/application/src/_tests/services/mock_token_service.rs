use domain::_shared::value_objects::Url;

use crate::{
    dtos::CurrentUserDto,
    services::TokenService,
};

pub struct MockTokenService {
    secret: String,
}

impl MockTokenService {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}

impl TokenService for MockTokenService {
    fn generate_token(&self, user_id: &str, _: &str, _: Option<Url>) -> anyhow::Result<String> {
        let _secret = &self.secret;
        Ok(user_id.to_string()) // In a real implementation, you would generate a JWT token here
    }

    fn verify_token(&self, token: &str) -> anyhow::Result<CurrentUserDto> {
        Ok(CurrentUserDto::new(uuid::Uuid::new_v4(), token.to_string(), None)) // In a real implementation, you would verify the JWT token and extract user information here
    }
}
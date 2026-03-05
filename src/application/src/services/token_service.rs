use crate::dtos::CurrentUserDto;

pub trait TokenService {
    fn generate_token(&self, user_id: &str, user_name: &str, user_avatar: Option<String>) -> anyhow::Result<String>;
    fn verify_token(&self, token: &str) -> anyhow::Result<CurrentUserDto>;
}
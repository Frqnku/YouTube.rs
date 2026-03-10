use domain::_shared::value_objects::Url;

use crate::dtos::CurrentUserDto;

pub trait TokenService {
    fn generate_token(&self, user_id: &str, user_name: &str, user_avatar: Option<Url>) -> anyhow::Result<String>;
    fn verify_token(&self, token: &str) -> anyhow::Result<CurrentUserDto>;
}
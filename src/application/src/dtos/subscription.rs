use domain::{_shared::value_objects::Url, user::entity::User};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelDto {
	pub id: Uuid,
	pub name: String,
	pub profile_picture: Option<Url>,
}

impl ChannelDto {
	pub fn new(id: Uuid, name: String, profile_picture: Option<Url>) -> Self {
		Self {
			id,
			name,
			profile_picture,
		}
	}
}

impl From<User> for ChannelDto {
	fn from(user: User) -> Self {
		Self::new(user.id, user.name, user.profile_picture)
	}
}

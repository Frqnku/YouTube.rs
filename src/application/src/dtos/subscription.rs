use domain::{_shared::value_objects::Url, channel::entity::Channel};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelDto {
	pub id: Uuid,
	pub name: String,
	pub profile_picture: Option<Url>,
	pub banner: Option<Url>,
}

impl ChannelDto {
	pub fn new(
		id: Uuid,
		name: String,
		profile_picture: Option<Url>,
		banner: Option<Url>,
	) -> Self {
		Self {
			id,
			name,
			profile_picture,
			banner,
		}
	}
}

impl From<Channel> for ChannelDto {
	fn from(channel: Channel) -> Self {
		Self::new(channel.id, channel.name, channel.profile_picture, channel.banner)
	}
}

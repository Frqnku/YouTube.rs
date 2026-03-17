use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use application::dtos::subscription::ChannelDto as AppChannelDto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelDto {
	pub id: String,
	pub name: String,
	pub profile_picture: Option<String>,
}

impl ChannelDto {
	pub fn new(id: String, name: String, profile_picture: Option<String>) -> Self {
		Self {
			id,
			name,
			profile_picture,
		}
	}
}

#[cfg(feature = "ssr")]
impl From<AppChannelDto> for ChannelDto {
	fn from(channel: AppChannelDto) -> Self {
		Self::new(
			channel.id.to_string(),
			channel.name,
			channel.profile_picture.map(|url| url.to_string()),
		)
	}
}

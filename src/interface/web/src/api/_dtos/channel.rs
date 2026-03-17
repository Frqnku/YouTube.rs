use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use application::dtos::channel::ChannelDataDto as AppChannelDataDto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelDataDto {
	pub id: String,
	pub name: String,
	pub profile_picture: Option<String>,
	pub description: Option<String>,
	pub subscriber_count: usize,
	pub video_count: usize,
}

impl ChannelDataDto {
	pub fn new(
		id: String,
		name: String,
		profile_picture: Option<String>,
		description: Option<String>,
		subscriber_count: usize,
		video_count: usize,
	) -> Self {
		Self {
			id,
			name,
			profile_picture,
			description,
			subscriber_count,
			video_count,
		}
	}
}

#[cfg(feature = "ssr")]
impl From<AppChannelDataDto> for ChannelDataDto {
	fn from(channel: AppChannelDataDto) -> Self {
		Self::new(
			channel.id.to_string(),
			channel.name,
			channel.profile_picture.map(|url| url.to_string()),
			channel.description,
			channel.subscriber_count,
			channel.video_count,
		)
	}
}
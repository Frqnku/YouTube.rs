use domain::{
	_shared::value_objects::Url,
	channel::entity::Channel,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelDataDto {
	pub id: Uuid,
	pub name: String,
	pub profile_picture: Option<Url>,
	pub banner: Option<Url>,
	pub description: Option<String>,
	pub subscriber_count: usize,
	pub video_count: usize,
}

impl ChannelDataDto {
	pub fn new(
		id: Uuid,
		name: String,
		profile_picture: Option<Url>,
		banner: Option<Url>,
		description: Option<String>,
		subscriber_count: usize,
		video_count: usize,
	) -> Self {
		Self {
			id,
			name,
			profile_picture,
			banner,
			description,
			subscriber_count,
			video_count,
		}
	}
}

impl From<Channel> for ChannelDataDto {
	fn from(channel: Channel) -> Self {
		Self::new(
			channel.id,
			channel.name,
			channel.profile_picture,
			channel.banner,
			channel.description,
			channel.subscriber_count,
			channel.video_count,
		)
	}
}
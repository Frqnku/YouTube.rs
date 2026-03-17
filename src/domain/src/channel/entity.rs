use uuid::Uuid;

use crate::_shared::value_objects::Url;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Channel {
	pub id: Uuid,
	pub name: String,
	pub profile_picture: Option<Url>,
	pub description: Option<String>,
	pub subscriber_count: usize,
	pub video_count: usize,
}

impl Channel {
	pub fn new(
		id: Uuid,
		name: String,
		profile_picture: Option<Url>,
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

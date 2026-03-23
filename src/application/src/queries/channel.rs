use crate::dtos::ChannelDataDto;
use crate::_helpers::parse_uuid;
use domain::{
	channel::ChannelRepository,
};

pub struct GetChannelData<
	'a,
	R: ChannelRepository,
> {
	pub channel_repository: &'a R,
}

impl<'a, R> GetChannelData<'a, R>
where
	R: ChannelRepository,
{
	pub async fn execute(&self, channel_id: String) -> anyhow::Result<Option<ChannelDataDto>> {
		let channel_id = parse_uuid(&channel_id, "channel id")?;

		let channel = self
			.channel_repository
			.find_channel_by_id(channel_id)
			.await?;

		Ok(channel.map(ChannelDataDto::from))
	}
}

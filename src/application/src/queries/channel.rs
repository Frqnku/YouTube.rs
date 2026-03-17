use crate::dtos::ChannelDataDto;
use domain::{
	_shared::DomainError,
	channel::ChannelRepository,
};
use uuid::Uuid;

fn parse_uuid(id: &str, field_name: &str) -> anyhow::Result<Uuid> {
	Uuid::parse_str(id)
		.map_err(|_| DomainError::BadRequest(format!("Invalid {field_name}")))
		.map_err(Into::into)
}

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

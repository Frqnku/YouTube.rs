use domain::{
	_shared::DomainError,
	video::VideoViewRepository,
};
use uuid::Uuid;

const RECOUNT_AFTER_SECONDS: i64 = 5 * 60;

fn parse_uuid(id: &str) -> anyhow::Result<Uuid> {
	Uuid::parse_str(id).map_err(|_| DomainError::VideoNotFound.into())
}

pub struct RegisterVideoView<
	'a,
	R: VideoViewRepository,
> {
	pub view_repository: &'a R,
}

impl<'a, R> RegisterVideoView<'a, R>
where
	R: VideoViewRepository,
{
	pub async fn execute(
		&self,
		video_id: String,
		user_id: Option<String>,
		ip_address: Option<String>,
	) -> anyhow::Result<()> {
		let video_id = parse_uuid(&video_id)?;
		let user_id = user_id
			.map(|id| parse_uuid(&id))
			.transpose()?;
		let ip_address = ip_address
			.map(|ip| ip.trim().to_string())
			.filter(|ip| !ip.is_empty());

		self
			.view_repository
			.register_view(video_id, user_id, ip_address, RECOUNT_AFTER_SECONDS)
			.await
	}
}

use domain::{
	_shared::DomainError,
	video::VideoReactionRepository,
};
use uuid::Uuid;

fn parse_uuid(id: &str) -> anyhow::Result<Uuid> {
	Uuid::parse_str(id).map_err(|_| DomainError::VideoNotFound.into())
}

pub struct GetVideoReactionStatus<
	'a,
	R: VideoReactionRepository,
> {
	pub reaction_repository: &'a R,
}

impl<'a, R> GetVideoReactionStatus<'a, R>
where
	R: VideoReactionRepository,
{
	pub async fn execute(&self, user_id: String, video_id: String) -> anyhow::Result<(bool, bool)> {
		let user_id = parse_uuid(&user_id)?;
		let video_id = parse_uuid(&video_id)?;

		self.reaction_repository
			.find_like_status(user_id, video_id)
			.await
	}
}

pub struct AddVideoLike<
	'a,
	R: VideoReactionRepository,
> {
	pub reaction_repository: &'a R,
}

impl<'a, R> AddVideoLike<'a, R>
where
	R: VideoReactionRepository,
{
	pub async fn execute(&self, user_id: String, video_id: String) -> anyhow::Result<()> {
		let user_id = parse_uuid(&user_id)?;
		let video_id = parse_uuid(&video_id)?;

		self.reaction_repository
			.add_like(user_id, video_id)
			.await
	}
}

pub struct RemoveVideoLike<
	'a,
	R: VideoReactionRepository,
> {
	pub reaction_repository: &'a R,
}

impl<'a, R> RemoveVideoLike<'a, R>
where
	R: VideoReactionRepository,
{
	pub async fn execute(&self, user_id: String, video_id: String) -> anyhow::Result<()> {
		let user_id = parse_uuid(&user_id)?;
		let video_id = parse_uuid(&video_id)?;

		self.reaction_repository
			.remove_like(user_id, video_id)
			.await
	}
}

pub struct AddVideoDislike<
	'a,
	R: VideoReactionRepository,
> {
	pub reaction_repository: &'a R,
}

impl<'a, R> AddVideoDislike<'a, R>
where
	R: VideoReactionRepository,
{
	pub async fn execute(&self, user_id: String, video_id: String) -> anyhow::Result<()> {
		let user_id = parse_uuid(&user_id)?;
		let video_id = parse_uuid(&video_id)?;

		self.reaction_repository
			.add_dislike(user_id, video_id)
			.await
	}
}

pub struct RemoveVideoDislike<
	'a,
	R: VideoReactionRepository,
> {
	pub reaction_repository: &'a R,
}

impl<'a, R> RemoveVideoDislike<'a, R>
where
	R: VideoReactionRepository,
{
	pub async fn execute(&self, user_id: String, video_id: String) -> anyhow::Result<()> {
		let user_id = parse_uuid(&user_id)?;
		let video_id = parse_uuid(&video_id)?;

		self.reaction_repository
			.remove_dislike(user_id, video_id)
			.await
	}
}

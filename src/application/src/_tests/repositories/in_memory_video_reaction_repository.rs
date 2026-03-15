use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use domain::video::VideoReactionRepository;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct VideoReactionEntry {
	pub user_id: Uuid,
	pub video_id: Uuid,
	pub is_liked: bool,
	pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct InMemoryVideoReactionRepository {
	pub reactions: Arc<Mutex<HashMap<(Uuid, Uuid), VideoReactionEntry>>>,
	pub like_counts: Arc<Mutex<HashMap<Uuid, i64>>>,
	pub dislike_counts: Arc<Mutex<HashMap<Uuid, i64>>>,
}

impl InMemoryVideoReactionRepository {
	pub fn new() -> Self {
		Self {
			reactions: Arc::new(Mutex::new(HashMap::new())),
			like_counts: Arc::new(Mutex::new(HashMap::new())),
			dislike_counts: Arc::new(Mutex::new(HashMap::new())),
		}
	}

	pub fn like_count(&self, video_id: Uuid) -> i64 {
		*self.like_counts.lock().unwrap().get(&video_id).unwrap_or(&0)
	}

	pub fn dislike_count(&self, video_id: Uuid) -> i64 {
		*self.dislike_counts.lock().unwrap().get(&video_id).unwrap_or(&0)
	}

	fn increment_count(counts: &Arc<Mutex<HashMap<Uuid, i64>>>, video_id: Uuid) {
		let mut counts = counts.lock().unwrap();
		*counts.entry(video_id).or_insert(0) += 1;
	}

	fn decrement_count(counts: &Arc<Mutex<HashMap<Uuid, i64>>>, video_id: Uuid) {
		let mut counts = counts.lock().unwrap();
		let entry = counts.entry(video_id).or_insert(0);
		*entry = (*entry - 1).max(0);
	}
}

#[async_trait::async_trait]
impl VideoReactionRepository for InMemoryVideoReactionRepository {
	async fn find_like_status(&self, user_id: Uuid, video_id: Uuid) -> anyhow::Result<(bool, bool)> {
		let reactions = self.reactions.lock().unwrap();
		let status = match reactions.get(&(user_id, video_id)) {
			Some(reaction) if reaction.is_liked => (true, false),
			Some(_) => (false, true),
			None => (false, false),
		};

		Ok(status)
	}

	async fn add_like(&self, user_id: Uuid, video_id: Uuid) -> anyhow::Result<()> {
		let mut reactions = self.reactions.lock().unwrap();
		match reactions.get_mut(&(user_id, video_id)) {
			Some(reaction) if reaction.is_liked => {}
			Some(reaction) => {
				reaction.is_liked = true;
				reaction.updated_at = chrono::Utc::now();
				Self::increment_count(&self.like_counts, video_id);
				Self::decrement_count(&self.dislike_counts, video_id);
			}
			None => {
				reactions.insert(
					(user_id, video_id),
					VideoReactionEntry {
						user_id,
						video_id,
						is_liked: true,
						updated_at: chrono::Utc::now(),
					},
				);
				Self::increment_count(&self.like_counts, video_id);
			}
		}

		Ok(())
	}

	async fn remove_like(&self, user_id: Uuid, video_id: Uuid) -> anyhow::Result<()> {
		let mut reactions = self.reactions.lock().unwrap();
		let should_remove = reactions
			.get(&(user_id, video_id))
			.map(|reaction| reaction.is_liked)
			.unwrap_or(false);

		if should_remove {
			reactions.remove(&(user_id, video_id));
			Self::decrement_count(&self.like_counts, video_id);
		}

		Ok(())
	}

	async fn add_dislike(&self, user_id: Uuid, video_id: Uuid) -> anyhow::Result<()> {
		let mut reactions = self.reactions.lock().unwrap();
		match reactions.get_mut(&(user_id, video_id)) {
			Some(reaction) if !reaction.is_liked => {}
			Some(reaction) => {
				reaction.is_liked = false;
				reaction.updated_at = chrono::Utc::now();
				Self::increment_count(&self.dislike_counts, video_id);
				Self::decrement_count(&self.like_counts, video_id);
			}
			None => {
				reactions.insert(
					(user_id, video_id),
					VideoReactionEntry {
						user_id,
						video_id,
						is_liked: false,
						updated_at: chrono::Utc::now(),
					},
				);
				Self::increment_count(&self.dislike_counts, video_id);
			}
		}

		Ok(())
	}

	async fn remove_dislike(&self, user_id: Uuid, video_id: Uuid) -> anyhow::Result<()> {
		let mut reactions = self.reactions.lock().unwrap();
		let should_remove = reactions
			.get(&(user_id, video_id))
			.map(|reaction| !reaction.is_liked)
			.unwrap_or(false);

		if should_remove {
			reactions.remove(&(user_id, video_id));
			Self::decrement_count(&self.dislike_counts, video_id);
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn test_add_like_sets_like_status() {
		let repository = InMemoryVideoReactionRepository::new();
		let user_id = Uuid::new_v4();
		let video_id = Uuid::new_v4();

		repository.add_like(user_id, video_id).await.unwrap();

		assert_eq!(repository.find_like_status(user_id, video_id).await.unwrap(), (true, false));
		assert_eq!(repository.like_count(video_id), 1);
	}

	#[tokio::test]
	async fn test_add_dislike_switches_existing_like() {
		let repository = InMemoryVideoReactionRepository::new();
		let user_id = Uuid::new_v4();
		let video_id = Uuid::new_v4();

		repository.add_like(user_id, video_id).await.unwrap();
		repository.add_dislike(user_id, video_id).await.unwrap();

		assert_eq!(repository.find_like_status(user_id, video_id).await.unwrap(), (false, true));
		assert_eq!(repository.like_count(video_id), 0);
		assert_eq!(repository.dislike_count(video_id), 1);
	}

	#[tokio::test]
	async fn test_remove_like_clears_reaction() {
		let repository = InMemoryVideoReactionRepository::new();
		let user_id = Uuid::new_v4();
		let video_id = Uuid::new_v4();

		repository.add_like(user_id, video_id).await.unwrap();
		repository.remove_like(user_id, video_id).await.unwrap();

		assert_eq!(repository.find_like_status(user_id, video_id).await.unwrap(), (false, false));
		assert_eq!(repository.like_count(video_id), 0);
	}
}

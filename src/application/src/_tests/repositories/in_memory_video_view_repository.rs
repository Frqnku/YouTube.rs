use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use chrono::{Duration, Utc};
use domain::video::VideoViewRepository;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct VideoViewEntry {
	pub video_id: Uuid,
	pub user_id: Option<Uuid>,
	pub ip_address: Option<String>,
	pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct InMemoryVideoViewRepository {
	pub views: Arc<Mutex<Vec<VideoViewEntry>>>,
	pub view_counts: Arc<Mutex<HashMap<Uuid, i64>>>,
}

impl InMemoryVideoViewRepository {
	pub fn new() -> Self {
		Self {
			views: Arc::new(Mutex::new(Vec::new())),
			view_counts: Arc::new(Mutex::new(HashMap::new())),
		}
	}

	fn latest_user_view_index(views: &[VideoViewEntry], video_id: Uuid, user_id: Uuid) -> Option<usize> {
		views
			.iter()
			.enumerate()
			.filter(|(_, view)| view.video_id == video_id && view.user_id == Some(user_id))
			.max_by_key(|(_, view)| view.updated_at)
			.map(|(index, _)| index)
	}

	fn latest_ip_view_index(views: &[VideoViewEntry], video_id: Uuid, ip_address: &str) -> Option<usize> {
		views
			.iter()
			.enumerate()
			.filter(|(_, view)| view.video_id == video_id && view.ip_address.as_deref() == Some(ip_address))
			.max_by_key(|(_, view)| view.updated_at)
			.map(|(index, _)| index)
	}

	fn increment_view_count(&self, video_id: Uuid) {
		let mut counts = self.view_counts.lock().unwrap();
		*counts.entry(video_id).or_insert(0) += 1;
	}

	pub fn view_count(&self, video_id: Uuid) -> i64 {
		*self.view_counts.lock().unwrap().get(&video_id).unwrap_or(&0)
	}

	fn can_recount(updated_at: chrono::DateTime<chrono::Utc>, recount_after_seconds: i64) -> bool {
		updated_at < Utc::now() - Duration::seconds(recount_after_seconds)
	}
}

#[async_trait::async_trait]
impl VideoViewRepository for InMemoryVideoViewRepository {
	async fn register_view(
		&self,
		video_id: Uuid,
		user_id: Option<Uuid>,
		ip_address: Option<String>,
		recount_after_seconds: i64,
	) -> anyhow::Result<()> {
		let now = Utc::now();
		let mut should_increment = true;
		let mut views = self.views.lock().unwrap();

		if let Some(user_id) = user_id {
			let existing_user_index = Self::latest_user_view_index(&views, video_id, user_id);
			let existing_ip_index = ip_address
				.as_deref()
				.and_then(|ip| Self::latest_ip_view_index(&views, video_id, ip));

			let ip_can_recount = existing_ip_index
				.map(|index| Self::can_recount(views[index].updated_at, recount_after_seconds))
				.unwrap_or(true);

			if let Some(index) = existing_user_index {
				let user_can_recount = Self::can_recount(views[index].updated_at, recount_after_seconds);
				if let Some(ip_address) = ip_address {
					views[index].ip_address = Some(ip_address);
				}
				views[index].updated_at = now;
				should_increment = user_can_recount && ip_can_recount;
			} else if let Some(index) = existing_ip_index {
				if views[index].user_id.is_none() {
					views[index].user_id = Some(user_id);
					if let Some(ip_address) = ip_address {
						views[index].ip_address = Some(ip_address);
					}
					views[index].updated_at = now;
				} else {
					views.push(VideoViewEntry {
						video_id,
						user_id: Some(user_id),
						ip_address,
						updated_at: now,
					});
				}
				should_increment = ip_can_recount;
			} else {
				views.push(VideoViewEntry {
					video_id,
					user_id: Some(user_id),
					ip_address,
					updated_at: now,
				});
			}
		} else if let Some(ip_address) = ip_address {
			if let Some(index) = Self::latest_ip_view_index(&views, video_id, &ip_address) {
				should_increment = Self::can_recount(views[index].updated_at, recount_after_seconds);
				views[index].updated_at = now;
				views[index].ip_address = Some(ip_address);
			} else {
				views.push(VideoViewEntry {
					video_id,
					user_id: None,
					ip_address: Some(ip_address),
					updated_at: now,
				});
			}
		} else {
			views.push(VideoViewEntry {
				video_id,
				user_id: None,
				ip_address: None,
				updated_at: now,
			});
		}

		drop(views);

		if should_increment {
			self.increment_view_count(video_id);
		}

		Ok(())
	}

    async fn update_watched_seconds(&self, _video_id: Uuid, _user_id: Uuid, _watched_seconds: u32) -> anyhow::Result<()> {
        // No-op for in-memory repository
        Ok(())
    }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn test_register_view_counts_first_view() {
		let repository = InMemoryVideoViewRepository::new();
		let video_id = Uuid::new_v4();

		repository
			.register_view(video_id, None, Some("127.0.0.1".to_string()), 300)
			.await
			.unwrap();

		assert_eq!(repository.view_count(video_id), 1);
		assert_eq!(repository.views.lock().unwrap().len(), 1);
	}

	#[tokio::test]
	async fn test_register_view_does_not_recount_recent_same_ip() {
		let repository = InMemoryVideoViewRepository::new();
		let video_id = Uuid::new_v4();

		repository
			.register_view(video_id, None, Some("127.0.0.1".to_string()), 300)
			.await
			.unwrap();
		repository
			.register_view(video_id, None, Some("127.0.0.1".to_string()), 300)
			.await
			.unwrap();

		assert_eq!(repository.view_count(video_id), 1);
	}

	#[tokio::test]
	async fn test_register_view_promotes_anonymous_ip_to_user() {
		let repository = InMemoryVideoViewRepository::new();
		let video_id = Uuid::new_v4();
		let user_id = Uuid::new_v4();

		repository
			.register_view(video_id, None, Some("127.0.0.1".to_string()), 300)
			.await
			.unwrap();
		repository
			.register_view(video_id, Some(user_id), Some("127.0.0.1".to_string()), 300)
			.await
			.unwrap();

		let views = repository.views.lock().unwrap();
		assert_eq!(views.len(), 1);
		assert_eq!(views[0].user_id, Some(user_id));
		assert_eq!(repository.view_count(video_id), 1);
	}
}

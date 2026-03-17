use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use anyhow::{Context, anyhow};
use domain::video::{LikedVideoRepository, PageRequest, VideoHistoryRepository, VideoPage, VideoRepository, entity::Video};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct VideoHistoryEntry {
	pub user_id: Uuid,
	pub video_id: Uuid,
	pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct LikedVideoEntry {
	pub user_id: Uuid,
	pub video_id: Uuid,
	pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct InMemoryVideoRepository {
	pub videos: Arc<Mutex<Vec<Video>>>,
	pub history_entries: Arc<Mutex<Vec<VideoHistoryEntry>>>,
	pub liked_video_entries: Arc<Mutex<Vec<LikedVideoEntry>>>,
}

impl InMemoryVideoRepository {
	pub fn new() -> Self {
		Self {
			videos: Arc::new(Mutex::new(Vec::new())),
			history_entries: Arc::new(Mutex::new(Vec::new())),
			liked_video_entries: Arc::new(Mutex::new(Vec::new())),
		}
	}
}

fn newest_cursor(video: &Video) -> String {
	format!("{}|{}", video.created_at.timestamp_nanos_opt().unwrap_or(0), video.id)
}

fn parse_newest_cursor(cursor: &str) -> anyhow::Result<(chrono::DateTime<chrono::Utc>, Uuid)> {
	let (ts, id) = cursor
		.split_once('|')
		.context("Invalid newest cursor format")?;

	let nanos: i64 = ts.parse().context("Invalid newest cursor timestamp")?;
	let secs = nanos.div_euclid(1_000_000_000);
	let sub_nanos = nanos.rem_euclid(1_000_000_000) as u32;

	let at = chrono::DateTime::<chrono::Utc>::from_timestamp(secs, sub_nanos)
		.ok_or_else(|| anyhow!("Invalid newest cursor timestamp range"))?;
	let id = Uuid::parse_str(id).context("Invalid newest cursor id")?;

	Ok((at, id))
}

fn popular_cursor(video: &Video) -> String {
	format!("{}|{}", video.view_count, video.id)
}

fn parse_popular_cursor(cursor: &str) -> anyhow::Result<(i64, Uuid)> {
	let (count, id) = cursor
		.split_once('|')
		.context("Invalid popular cursor format")?;

	let count: i64 = count.parse().context("Invalid popular cursor view count")?;
	let id = Uuid::parse_str(id).context("Invalid popular cursor id")?;

	Ok((count, id))
}

fn cmp_newest(a: &Video, b: &Video) -> Ordering {
	b.created_at
		.cmp(&a.created_at)
		.then_with(|| b.id.cmp(&a.id))
}

fn cmp_popular(a: &Video, b: &Video) -> Ordering {
	b.view_count
		.cmp(&a.view_count)
		.then_with(|| b.id.cmp(&a.id))
}

fn cmp_updated_then_id(
	a: &(Video, chrono::DateTime<chrono::Utc>),
	b: &(Video, chrono::DateTime<chrono::Utc>),
) -> Ordering {
	b.1
		.cmp(&a.1)
		.then_with(|| b.0.id.cmp(&a.0.id))
}

fn build_page<F>(mut items: Vec<Video>, page: PageRequest, cursor_fn: F) -> anyhow::Result<VideoPage>
where
	F: Fn(&Video) -> String,
{
	let limit = usize::try_from(page.limit).context("Invalid page limit")?;
	let has_more = items.len() > limit;

	if has_more {
		items.truncate(limit);
	}

	let next_cursor = if has_more {
		items.last().map(cursor_fn)
	} else {
		None
	};

	Ok(VideoPage::new(items, next_cursor, has_more))
}

fn build_updated_page(
	mut items: Vec<(Video, chrono::DateTime<chrono::Utc>)>,
	page: PageRequest,
) -> anyhow::Result<VideoPage> {
	let limit = usize::try_from(page.limit).context("Invalid page limit")?;
	let has_more = items.len() > limit;

	if has_more {
		items.truncate(limit);
	}

	let next_cursor = if has_more {
		items.last().map(|(video, updated_at)| {
			format!("{}|{}", updated_at.timestamp_nanos_opt().unwrap_or(0), video.id)
		})
	} else {
		None
	};

	let videos = items.into_iter().map(|(video, _)| video).collect();

	Ok(VideoPage::new(videos, next_cursor, has_more))
}

#[async_trait::async_trait]
impl VideoRepository for InMemoryVideoRepository {
	async fn find_by_id(&self, id: Uuid, _viewer_user_id: Option<Uuid>) -> Option<Video> {
		self.videos
			.lock()
			.unwrap()
			.iter()
			.find(|video| video.id == id)
			.cloned()
	}

	async fn list_newest(&self, page: PageRequest, _viewer_user_id: Option<Uuid>) -> anyhow::Result<VideoPage> {
		let mut items = self.videos.lock().unwrap().clone();
		items.sort_by(cmp_newest);

		if let Some(cursor) = page.cursor.as_deref() {
			let (created_at, id) = parse_newest_cursor(cursor)?;
			items.retain(|video| {
				video.created_at < created_at || (video.created_at == created_at && video.id < id)
			});
		}

		let limit_plus_one = usize::try_from(page.limit.saturating_add(1)).context("Invalid page limit")?;
		if items.len() > limit_plus_one {
			items.truncate(limit_plus_one);
		}

		build_page(items, page, newest_cursor)
	}

	async fn list_most_popular(&self, page: PageRequest, _viewer_user_id: Option<Uuid>) -> anyhow::Result<VideoPage> {
		let mut items = self.videos.lock().unwrap().clone();
		items.sort_by(cmp_popular);

		if let Some(cursor) = page.cursor.as_deref() {
			let (view_count, id) = parse_popular_cursor(cursor)?;
			items.retain(|video| {
				video.view_count < view_count || (video.view_count == view_count && video.id < id)
			});
		}

		let limit_plus_one = usize::try_from(page.limit.saturating_add(1)).context("Invalid page limit")?;
		if items.len() > limit_plus_one {
			items.truncate(limit_plus_one);
		}

		build_page(items, page, popular_cursor)
	}

	async fn list_random(&self, page: PageRequest, _viewer_user_id: Option<Uuid>) -> anyhow::Result<VideoPage> {
		let mut items = self.videos.lock().unwrap().clone();
		let len = items.len();

		if len == 0 {
			return Ok(VideoPage::new(Vec::new(), None, false));
		}

		let seed = chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default().unsigned_abs() as usize;
		let rotate_by = seed % len;
		items.rotate_left(rotate_by);

		let limit = usize::try_from(page.limit).context("Invalid page limit")?;
		if items.len() > limit {
			items.truncate(limit);
		}

		Ok(VideoPage::new(items, None, false))
	}

	async fn list_by_user_id(&self, user_id: Uuid, page: PageRequest, _viewer_user_id: Option<Uuid>) -> anyhow::Result<VideoPage> {
		let mut items = self
			.videos
			.lock()
			.unwrap()
			.iter()
			.filter(|video| video.author.id == user_id)
			.cloned()
			.collect::<Vec<_>>();
		items.sort_by(cmp_newest);

		if let Some(cursor) = page.cursor.as_deref() {
			let (created_at, id) = parse_newest_cursor(cursor)?;
			items.retain(|video| {
				video.created_at < created_at || (video.created_at == created_at && video.id < id)
			});
		}

		let limit_plus_one = usize::try_from(page.limit.saturating_add(1)).context("Invalid page limit")?;
		if items.len() > limit_plus_one {
			items.truncate(limit_plus_one);
		}

		build_page(items, page, newest_cursor)
	}

	async fn list_by_tag(&self, tag_name: &str, page: PageRequest, _viewer_user_id: Option<Uuid>) -> anyhow::Result<VideoPage> {
		let mut items = self
			.videos
			.lock()
			.unwrap()
			.iter()
			.filter(|video| video.tags.iter().any(|tag| tag.name == tag_name))
			.cloned()
			.collect::<Vec<_>>();
		items.sort_by(cmp_newest);

		if let Some(cursor) = page.cursor.as_deref() {
			let (created_at, id) = parse_newest_cursor(cursor)?;
			items.retain(|video| {
				video.created_at < created_at || (video.created_at == created_at && video.id < id)
			});
		}

		let limit_plus_one = usize::try_from(page.limit.saturating_add(1)).context("Invalid page limit")?;
		if items.len() > limit_plus_one {
			items.truncate(limit_plus_one);
		}

		build_page(items, page, newest_cursor)
	}

	async fn count_by_user_id(&self, user_id: Uuid) -> anyhow::Result<u64> {
		let count = self
			.videos
			.lock()
			.unwrap()
			.iter()
			.filter(|video| video.author.id == user_id)
			.count();

		Ok(count as u64)
	}

	async fn search_by_title(&self, query: &str, page: PageRequest, _viewer_user_id: Option<Uuid>) -> anyhow::Result<VideoPage> {
		let needle = query.to_lowercase();

		let mut items = self
			.videos
			.lock()
			.unwrap()
			.iter()
			.filter(|video| {
				video.title.to_lowercase().contains(&needle)
					|| video.author.name.to_lowercase().contains(&needle)
			})
			.cloned()
			.collect::<Vec<_>>();
		items.sort_by(cmp_newest);

		if let Some(cursor) = page.cursor.as_deref() {
			let (created_at, id) = parse_newest_cursor(cursor)?;
			items.retain(|video| {
				video.created_at < created_at || (video.created_at == created_at && video.id < id)
			});
		}

		let limit_plus_one = usize::try_from(page.limit.saturating_add(1)).context("Invalid page limit")?;
		if items.len() > limit_plus_one {
			items.truncate(limit_plus_one);
		}

		build_page(items, page, newest_cursor)
	}

	async fn save(&self, video: &Video) -> anyhow::Result<Video> {
		let new_video = video.clone();
		self.videos.lock().unwrap().push(new_video.clone());

		Ok(new_video)
	}
}

#[async_trait::async_trait]
impl VideoHistoryRepository for InMemoryVideoRepository {
	async fn list_history_by_user_id(&self, user_id: Uuid, page: PageRequest) -> anyhow::Result<VideoPage> {
		let videos = self.videos.lock().unwrap().clone();
		let by_video_id = videos
			.into_iter()
			.map(|video| (video.id, video))
			.collect::<HashMap<_, _>>();

		let history_entries = self.history_entries.lock().unwrap().clone();
		let mut latest_by_video = HashMap::<Uuid, chrono::DateTime<chrono::Utc>>::new();

		for entry in history_entries.into_iter().filter(|entry| entry.user_id == user_id) {
			latest_by_video
				.entry(entry.video_id)
				.and_modify(|existing| {
					if entry.updated_at > *existing {
						*existing = entry.updated_at;
					}
				})
				.or_insert(entry.updated_at);
		}

		let mut items = latest_by_video
			.into_iter()
			.filter_map(|(video_id, updated_at)| {
				by_video_id
					.get(&video_id)
					.cloned()
					.map(|video| (video, updated_at))
			})
			.collect::<Vec<_>>();
		items.sort_by(cmp_updated_then_id);

		if let Some(cursor) = page.cursor.as_deref() {
			let (updated_at, id) = parse_newest_cursor(cursor)?;
			items.retain(|(video, item_updated_at)| {
				*item_updated_at < updated_at || (*item_updated_at == updated_at && video.id < id)
			});
		}

		let limit_plus_one = usize::try_from(page.limit.saturating_add(1)).context("Invalid page limit")?;
		if items.len() > limit_plus_one {
			items.truncate(limit_plus_one);
		}

		build_updated_page(items, page)
	}
}

#[async_trait::async_trait]
impl LikedVideoRepository for InMemoryVideoRepository {
	async fn list_liked_videos_by_user_id(&self, user_id: Uuid, page: PageRequest) -> anyhow::Result<VideoPage> {
		let videos = self.videos.lock().unwrap().clone();
		let by_video_id = videos
			.into_iter()
			.map(|video| (video.id, video))
			.collect::<HashMap<_, _>>();

		let liked_entries = self.liked_video_entries.lock().unwrap().clone();
		let mut latest_by_video = HashMap::<Uuid, chrono::DateTime<chrono::Utc>>::new();

		for entry in liked_entries.into_iter().filter(|entry| entry.user_id == user_id) {
			latest_by_video
				.entry(entry.video_id)
				.and_modify(|existing| {
					if entry.updated_at > *existing {
						*existing = entry.updated_at;
					}
				})
				.or_insert(entry.updated_at);
		}

		let mut items = latest_by_video
			.into_iter()
			.filter_map(|(video_id, updated_at)| {
				by_video_id
					.get(&video_id)
					.cloned()
					.map(|video| (video, updated_at))
			})
			.collect::<Vec<_>>();
		items.sort_by(cmp_updated_then_id);

		if let Some(cursor) = page.cursor.as_deref() {
			let (updated_at, id) = parse_newest_cursor(cursor)?;
			items.retain(|(video, item_updated_at)| {
				*item_updated_at < updated_at || (*item_updated_at == updated_at && video.id < id)
			});
		}

		let limit_plus_one = usize::try_from(page.limit.saturating_add(1)).context("Invalid page limit")?;
		if items.len() > limit_plus_one {
			items.truncate(limit_plus_one);
		}

		build_updated_page(items, page)
	}
}

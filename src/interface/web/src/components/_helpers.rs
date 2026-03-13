use chrono::{DateTime, Utc};

#[cfg(target_arch = "wasm32")]
const LOAD_MORE_THRESHOLD_PX: f64 = 1.0;

pub fn format_duration(total_seconds: i32) -> String {
	let safe_seconds = total_seconds.max(0);
	let hours = safe_seconds / 3_600;
	let minutes = (safe_seconds % 3_600) / 60;
	let seconds = safe_seconds % 60;

	if hours > 0 {
		format!("{hours}:{minutes:02}:{seconds:02}")
	} else {
		format!("{minutes}:{seconds:02}")
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CountFormat {
	Short,
	Long,
}

pub fn format_count(count: i64, format: CountFormat) -> String {
	if format == CountFormat::Long {
		let s = count.to_string();
		let mut result = String::new();
		let mut counter = 0;

		for c in s.chars().rev() {
			if counter != 0 && counter % 3 == 0 {
				result.push(',');
			}
			result.push(c);
			counter += 1;
		}

		return result.chars().rev().collect();
	}

	match count {
		0..=999 => count.to_string(),
		1_000..=999_999 => {
			let thousands = count as f64 / 1_000.0;
			if thousands >= 100.0 {
				format!("{thousands:.0}K")
			} else {
				format!("{thousands:.1}K")
			}
		}
		1_000_000..=999_999_999 => {
			let millions = count as f64 / 1_000_000.0;
			if millions >= 100.0 {
				format!("{millions:.0}M")
			} else {
				format!("{millions:.1}M")
			}
		}
		_ => {
			let billions = count as f64 / 1_000_000_000.0;
			format!("{billions:.1}B")
		}
	}
}

pub fn format_relative_time(timestamp: &str) -> String {
	let Ok(parsed) = DateTime::parse_from_rfc3339(timestamp) else {
		return "just now".to_string();
	};

	let now = Utc::now();
	let published_at = parsed.with_timezone(&Utc);
	let delta = now.signed_duration_since(published_at);

	if delta.num_minutes() < 1 {
		return "just now".to_string();
	}

	if delta.num_minutes() < 60 {
		let minutes = delta.num_minutes();
		return if minutes == 1 {
			"1 minute ago".to_string()
		} else {
			format!("{minutes} minutes ago")
		};
	}

	if delta.num_hours() < 24 {
		let hours = delta.num_hours();
		return if hours == 1 {
			"1 hour ago".to_string()
		} else {
			format!("{hours} hours ago")
		};
	}

	if delta.num_days() < 7 {
		let days = delta.num_days();
		return if days == 1 {
			"1 day ago".to_string()
		} else {
			format!("{days} days ago")
		};
	}

	if delta.num_days() < 30 {
		let weeks = delta.num_days() / 7;
		return if weeks == 1 {
			"1 week ago".to_string()
		} else {
			format!("{weeks} weeks ago")
		};
	}

	if delta.num_days() < 365 {
		let months = delta.num_days() / 30;
		return if months == 1 {
			"1 month ago".to_string()
		} else {
			format!("{months} months ago")
		};
	}

	let years = delta.num_days() / 365;
	if years == 1 {
		"1 year ago".to_string()
	} else {
		format!("{years} years ago")
	}
}

#[cfg(target_arch = "wasm32")]
pub fn is_near_bottom_of_page() -> bool {
	let Some(window) = web_sys::window() else {
		return false;
	};

	let Some(document) = window.document() else {
		return false;
	};

	let scroll_y = window.scroll_y().ok().unwrap_or(0.0);
	let viewport_height = window
		.inner_height()
		.ok()
		.and_then(|value| value.as_f64())
		.unwrap_or(0.0);

	let page_height = document
		.document_element()
		.map(|element| element.scroll_height() as f64)
		.or_else(|| document.body().map(|body| body.scroll_height() as f64))
		.unwrap_or(0.0);

	let remaining = page_height - (scroll_y + viewport_height);
	remaining <= LOAD_MORE_THRESHOLD_PX
}

#[cfg(not(target_arch = "wasm32"))]
pub fn is_near_bottom_of_page() -> bool {
	false
}

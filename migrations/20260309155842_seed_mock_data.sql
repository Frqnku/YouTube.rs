-- Migration: Seed mock data

-- =========================
-- Users mock data
-- =========================

INSERT INTO users (name, email, profile_picture)
VALUES
	('Rick Astley', 'rick.astley@example.com', 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg'),
	('Mr Beast', 'mr.beast@example.com', 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg'),
	('Linus Tech Tips', 'linus.tech@example.com', 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg'),
	('Marques Brownlee', 'marques.brownlee@example.com', 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg'),
	('PewDiePie', 'pewdiepie@example.com', 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg'),
	('Dude Perfect', 'dude.perfect@example.com', 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg'),
	('Vsauce', 'vsauce@example.com', 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg'),
	('Veritasium', 'veritasium@example.com', 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg'),
	('Kurzgesagt', 'kurzgesagt@example.com', 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg'),
	('Ali Abdaal', 'ali.abdaal@example.com', 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg'),
	('Mr. Innovator', 'mr.innovator@example.com', 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg'),
	('Tech Insider', 'tech.insider@example.com', 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg'),
	('The Coding Train', 'coding.train@example.com', 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg'),
	('Crash Course', 'crash.course@example.com', 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg'),
	('Wired', 'wired.channel@example.com', 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg'),
	('National Geographic', 'natgeo@example.com', 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg'),
	('TED-Ed', 'ted.ed@example.com', 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg')
ON CONFLICT (email) DO UPDATE
SET
	name = EXCLUDED.name,
	profile_picture = EXCLUDED.profile_picture,
	updated_at = now();

-- =========================
-- Videos mock data
-- =========================

INSERT INTO videos (user_id, title, description, duration_seconds, thumbnail_url, video_url, view_count, like_count, dislike_count, created_at)
SELECT
	u.id,
	v.title,
	v.description,
	v.duration_seconds,
	v.thumbnail_url,
	v.video_url,
	v.view_count,
	v.like_count,
	v.dislike_count,
	v.created_at
FROM users u
JOIN (
	VALUES
		('rick.astley@example.com', 'Never Gonna Give You Up', 'Official music video remaster', 213, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4', 18523847, 368477, 1852, '2024-03-22 14:37:09+00'::timestamptz),
		('rick.astley@example.com', 'Together Forever Live', 'Live performance from world tour', 267, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4', 23875, 478, 2, '2025-11-08 09:12:44+00'::timestamptz),
		('mr.beast@example.com', 'I Gave Away 100 Cars', 'Biggest giveaway challenge of the year', 845, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4', 158479, 3136, 13, '2025-06-14 17:04:28+00'::timestamptz),
		('mr.beast@example.com', 'Last To Leave Wins 1M', 'High energy challenge with huge rewards', 1230, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4', 1924815, 38483, 198, '2024-09-27 11:51:03+00'::timestamptz),
		('linus.tech@example.com', 'Ultimate PC Build Guide 2026', 'Full beginner to pro PC build walkthrough', 1320, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4', 4562839, 91345, 457, '2025-01-09 20:33:17+00'::timestamptz),
		('marques.brownlee@example.com', 'Studio Tour 2026', 'Updated camera gear and production workflow', 540, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4', 319874, 6321, 39, '2025-08-17 16:08:52+00'::timestamptz),
		('pewdiepie@example.com', 'Gaming Highlights Weekly', 'Best moments and funny clips this week', 655, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4', 896342, 17923, 89, '2024-11-03 22:47:35+00'::timestamptz),
		('pewdiepie@example.com', 'Meme Review Throwback', 'Classic community memes and commentary', 430, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4', 72476, 1423, 7, '2025-04-22 13:29:41+00'::timestamptz),
		('dude.perfect@example.com', 'Impossible Trick Shots', 'Team challenge with new trick shots', 510, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4', 5621487, 112486, 563, '2024-07-05 08:15:27+00'::timestamptz),
		('dude.perfect@example.com', 'Sports Battle 2026', 'Competitive mini games and final showdown', 720, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4', 683492, 13674, 62, '2025-09-30 19:56:14+00'::timestamptz),
		('vsauce@example.com', 'Why Time Feels Fast', 'A deep dive into perception and memory', 1140, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4', 9187634, 183652, 918, '2024-04-18 10:22:59+00'::timestamptz),
		('vsauce@example.com', 'What Is Infinity', 'Math concepts explained with visual examples', 1260, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4', 112478, 2056, 10, '2025-12-07 07:44:06+00'::timestamptz),
		('veritasium@example.com', 'The Most Misunderstood Experiment', 'Physics experiment explained from first principles', 1015, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4', 872356, 17489, 87, '2024-10-29 15:38:22+00'::timestamptz),
		('veritasium@example.com', 'Can You Trust Statistics', 'How data can mislead and how to read it right', 890, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4', 786924, 15384, 87, '2025-02-14 12:07:48+00'::timestamptz),
		('kurzgesagt@example.com', 'The Future of Energy', 'Animated explainer on sustainable energy paths', 600, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4', 692567, 13891, 63, '2025-05-23 18:41:33+00'::timestamptz),
		('kurzgesagt@example.com', 'Inside Black Holes', 'Visual story about gravity and spacetime', 570, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4', 11287432, 225748, 1129, '2024-06-11 21:03:57+00'::timestamptz),
		('ali.abdaal@example.com', 'Productive Study Routine', 'Practical system for deep focused sessions', 740, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4', 2798456, 55968, 280, '2024-12-19 09:58:14+00'::timestamptz),
		('ali.abdaal@example.com', 'How I Plan My Week', 'Simple planning method for consistent output', 520, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4', 1956234, 39125, 196, '2025-07-08 14:25:39+00'::timestamptz),
		('mr.innovator@example.com', 'Top AI Tools in 2026', 'Hands on review of useful AI workflows', 680, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4', 408754, 8192, 49, '2025-10-31 11:17:52+00'::timestamptz),
		('mr.innovator@example.com', 'Automation for Creators', 'Build repeatable publishing pipelines', 610, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4', 3423876, 68476, 342, '2024-08-02 16:49:08+00'::timestamptz),
		('tech.insider@example.com', 'Future Gadgets You Will Want', 'A curated list of upcoming consumer tech', 555, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4', 528937, 10586, 52, '2025-03-16 08:32:47+00'::timestamptz),
		('tech.insider@example.com', 'How Chips Are Made', 'Factory process and design explained simply', 780, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4', 82341, 1691, 83, '2024-05-27 20:14:31+00'::timestamptz),
		('coding.train@example.com', 'Creative Coding with Rust', 'Build interactive visuals using Rust and wasm', 930, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4', 213456, 4691, 24, '2025-11-24 17:06:19+00'::timestamptz),
		('coding.train@example.com', 'Perlin Noise Playground', 'Step by step generative art techniques', 860, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4', 176542, 3530, 17, '2026-01-13 10:53:04+00'::timestamptz),
		('crash.course@example.com', 'History in 20 Minutes', 'Fast paced overview of key events', 1210, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4', 639875, 12795, 63, '2024-04-07 13:41:26+00'::timestamptz),
		('crash.course@example.com', 'Biology Basics', 'Cell structure and systems made simple', 990, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4', 51324, 1948, 63, '2025-08-29 07:28:55+00'::timestamptz),
		('wired.channel@example.com', 'Interview with a Robotics Engineer', 'Career path tools and real project lessons', 640, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4', 3678945, 73579, 368, '2024-09-14 18:03:37+00'::timestamptz),
		('wired.channel@example.com', 'Tech Support Challenge', 'Experts answer difficult internet questions', 700, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4', 4932187, 98643, 493, '2025-06-03 22:19:43+00'::timestamptz),
		('natgeo@example.com', 'Wildlife in the Arctic', 'Field footage of adaptation in extreme climates', 840, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4', 762893, 12578, 72, '2024-11-17 15:47:11+00'::timestamptz),
		('natgeo@example.com', 'Deep Ocean Mysteries', 'Exploration mission through remote habitats', 960, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4', 9745123, 194902, 974, '2024-03-29 09:34:58+00'::timestamptz),
		('ted.ed@example.com', 'How Memory Works', 'Animated lesson on memory formation and recall', 430, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4', 391876, 7437, 39, '2025-04-09 12:55:22+00'::timestamptz),
		('ted.ed@example.com', 'The Science of Sleep', 'What sleep does for brain and body performance', 450, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4', 4187234, 83745, 418, '2025-12-21 19:08:47+00'::timestamptz)
) AS v(uploader_email, title, description, duration_seconds, thumbnail_url, video_url, view_count, like_count, dislike_count, created_at)
	ON u.email = v.uploader_email
WHERE NOT EXISTS (
	SELECT 1
	FROM videos existing
	WHERE existing.user_id = u.id
	  AND existing.title = v.title
);

-- =========================
-- End of migration
-- =========================
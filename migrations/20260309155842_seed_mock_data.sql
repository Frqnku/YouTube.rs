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

INSERT INTO videos (user_id, title, description, duration_seconds, thumbnail_url, video_url)
SELECT
	u.id,
	v.title,
	v.description,
	v.duration_seconds,
	v.thumbnail_url,
	v.video_url
FROM users u
JOIN (
	VALUES
		('rick.astley@example.com', 'Never Gonna Give You Up', 'Official music video remaster', 213, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4'),
		('rick.astley@example.com', 'Together Forever Live', 'Live performance from world tour', 267, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4'),
		('mr.beast@example.com', 'I Gave Away 100 Cars', 'Biggest giveaway challenge of the year', 845, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4'),
		('mr.beast@example.com', 'Last To Leave Wins 1M', 'High energy challenge with huge rewards', 1230, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4'),
		('linus.tech@example.com', 'Ultimate PC Build Guide 2026', 'Full beginner to pro PC build walkthrough', 1320, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4'),
		('linus.tech@example.com', 'RTX Build Review', 'Real world thermals and gaming benchmarks', 980, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4'),
		('marques.brownlee@example.com', 'Best Smartphones 2026', 'My top picks after months of testing', 760, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4'),
		('marques.brownlee@example.com', 'Studio Tour 2026', 'Updated camera gear and production workflow', 540, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4'),
		('pewdiepie@example.com', 'Gaming Highlights Weekly', 'Best moments and funny clips this week', 655, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4'),
		('pewdiepie@example.com', 'Meme Review Throwback', 'Classic community memes and commentary', 430, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4'),
		('dude.perfect@example.com', 'Impossible Trick Shots', 'Team challenge with new trick shots', 510, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4'),
		('dude.perfect@example.com', 'Sports Battle 2026', 'Competitive mini games and final showdown', 720, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4'),
		('vsauce@example.com', 'Why Time Feels Fast', 'A deep dive into perception and memory', 1140, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4'),
		('vsauce@example.com', 'What Is Infinity', 'Math concepts explained with visual examples', 1260, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4'),
		('veritasium@example.com', 'The Most Misunderstood Experiment', 'Physics experiment explained from first principles', 1015, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4'),
		('veritasium@example.com', 'Can You Trust Statistics', 'How data can mislead and how to read it right', 890, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4'),
		('kurzgesagt@example.com', 'The Future of Energy', 'Animated explainer on sustainable energy paths', 600, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4'),
		('kurzgesagt@example.com', 'Inside Black Holes', 'Visual story about gravity and spacetime', 570, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4'),
		('ali.abdaal@example.com', 'Productive Study Routine', 'Practical system for deep focused sessions', 740, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4'),
		('ali.abdaal@example.com', 'How I Plan My Week', 'Simple planning method for consistent output', 520, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4'),
		('mr.innovator@example.com', 'Top AI Tools in 2026', 'Hands on review of useful AI workflows', 680, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4'),
		('mr.innovator@example.com', 'Automation for Creators', 'Build repeatable publishing pipelines', 610, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4'),
		('tech.insider@example.com', 'Future Gadgets You Will Want', 'A curated list of upcoming consumer tech', 555, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4'),
		('tech.insider@example.com', 'How Chips Are Made', 'Factory process and design explained simply', 780, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4'),
		('coding.train@example.com', 'Creative Coding with Rust', 'Build interactive visuals using Rust and wasm', 930, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4'),
		('coding.train@example.com', 'Perlin Noise Playground', 'Step by step generative art techniques', 860, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4'),
		('crash.course@example.com', 'History in 20 Minutes', 'Fast paced overview of key events', 1210, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4'),
		('crash.course@example.com', 'Biology Basics', 'Cell structure and systems made simple', 990, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4'),
		('wired.channel@example.com', 'Interview with a Robotics Engineer', 'Career path tools and real project lessons', 640, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4'),
		('wired.channel@example.com', 'Tech Support Challenge', 'Experts answer difficult internet questions', 700, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4'),
		('natgeo@example.com', 'Wildlife in the Arctic', 'Field footage of adaptation in extreme climates', 840, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4'),
		('natgeo@example.com', 'Deep Ocean Mysteries', 'Exploration mission through remote habitats', 960, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4'),
		('ted.ed@example.com', 'How Memory Works', 'Animated lesson on memory formation and recall', 430, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4'),
		('ted.ed@example.com', 'The Science of Sleep', 'What sleep does for brain and body performance', 450, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', '/videos/rickroll.mp4')
) AS v(uploader_email, title, description, duration_seconds, thumbnail_url, video_url)
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
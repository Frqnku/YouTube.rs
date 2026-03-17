-- Migration: Seed mock data

-- =========================
-- Users mock data
-- =========================

INSERT INTO users (name, email, profile_picture)
VALUES
	('Rick Astley', 'rick.astley@example.com', 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg'),
	('Mr Beast', 'mr.beast@example.com', 'https://yt3.googleusercontent.com/nxYrc_1_2f77DoBadyxMTmv7ZpRZapHR5jbuYe7PlPd5cIRJxtNNEYyOC0ZsxaDyJJzXrnJiuDE=s160-c-k-c0x00ffffff-no-rj'),
	('PewDiePie', 'pewdiepie@example.com', 'https://yt3.googleusercontent.com/vik8mAiwHQbXiFyKfZ3__p55_VBdGvwxPpuPJBBwdbF0PjJxikXhrP-C3nLQAMAxGNd_-xQCIg=s160-c-k-c0x00ffffff-no-rj'),
	('Fireship', 'fireship@example.com', 'https://yt3.googleusercontent.com/3fPNbkf_xPyCleq77ZhcxyeorY97NtMHVNUbaAON_RBDH9ydL4hJkjxC8x_4mpuopkB8oI7Ct6Y=s160-c-k-c0x00ffffff-no-rj'),
	('Netflix Japan', 'netflix@example.com', 'https://yt3.googleusercontent.com/VvPFaxHYEZxWbNaqFfWcd0whAzwNZvEwnw5WYlGzsXyqdj7mxdeO5-RqDlRnlh9F8JYXkHdbXw=s160-c-k-c0x00ffffff-no-rj'),
	('Squeezie', 'squeezie@example.com', 'https://yt3.googleusercontent.com/ytc/AIdro_mPZvx-xk6pbAYdC7G8jUZzgCNDDTg1ZfF0_Lwd8UpJT4M=s160-c-k-c0x00ffffff-no-rj'),
	('Low Level', 'low.level@example.com', 'https://yt3.googleusercontent.com/npUuy7Y37eUqanNoISokGb5a5rk1wotnx0Yzs1kfdOzYgnlwYnV82izSKmaoYpObYewKOAacs8o=s160-c-k-c0x00ffffff-no-rj'),
	('TED-Ed', 'ted.ed@example.com', 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg'),
	-- New comment-only users with random avatars
	('Alex Chen', 'alex.chen@example.com', 'https://i.pravatar.cc/150?img=1'),
	('Sarah Williams', 'sarah.w@example.com', 'https://i.pravatar.cc/150?img=2'),
	('James Rodriguez', 'james.r@example.com', 'https://i.pravatar.cc/150?img=3'),
	('Emma Thompson', 'emma.t@example.com', 'https://i.pravatar.cc/150?img=4'),
	('Michael Kumar', 'michael.k@example.com', 'https://i.pravatar.cc/150?img=5'),
	('Lisa Anderson', 'lisa.a@example.com', 'https://i.pravatar.cc/150?img=6'),
	('David Johnson', 'david.j@example.com', 'https://i.pravatar.cc/150?img=7'),
	('Maria Garcia', 'maria.g@example.com', 'https://i.pravatar.cc/150?img=8'),
	('Chris Lee', 'chris.l@example.com', 'https://i.pravatar.cc/150?img=9'),
	('Jessica Brown', 'jessica.b@example.com', 'https://i.pravatar.cc/150?img=10'),
	('Robert Zhang', 'robert.z@example.com', 'https://i.pravatar.cc/150?img=11'),
	('Anna Kowalski', 'anna.k@example.com', 'https://i.pravatar.cc/150?img=12'),
	('Kevin Wong', 'kevin.w@example.com', 'https://i.pravatar.cc/150?img=13'),
	('Sophie Martin', 'sophie.m@example.com', 'https://i.pravatar.cc/150?img=14'),
	('Daniel Patel', 'daniel.p@example.com', 'https://i.pravatar.cc/150?img=15'),
	('Natalie Fisher', 'natalie.f@example.com', 'https://i.pravatar.cc/150?img=16'),
	('Tom Brady', 'tom.brady@example.com', 'https://i.pravatar.cc/150?img=17'),
	('Rachel Green', 'rachel.green@example.com', 'https://i.pravatar.cc/150?img=18'),
	('Marcus Davis', 'marcus.d@example.com', 'https://i.pravatar.cc/150?img=19'),
	('Olivia Martinez', 'olivia.m@example.com', 'https://i.pravatar.cc/150?img=20')
ON CONFLICT (email) DO UPDATE
SET
	name = EXCLUDED.name,
	profile_picture = EXCLUDED.profile_picture,
	updated_at = now();

-- =========================
-- Videos mock data
-- =========================

INSERT INTO videos (user_id, title, description, duration_seconds, thumbnail_url, preview_url, video_url, view_count, like_count, dislike_count, created_at)
SELECT
	u.id,
	v.title,
	v.description,
	v.duration_seconds,
	v.thumbnail_url,
	v.preview_url,
	v.video_url,
	v.view_count,
	v.like_count,
	v.dislike_count,
	v.created_at
FROM users u
JOIN (
    VALUES
        ('rick.astley@example.com', 'Never Gonna Give You Up', 'Official music video remaster', 213, 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg', 'https://de8x082p75eq8.cloudfront.net/previews/rick.webp', 'https://de8x082p75eq8.cloudfront.net/videos/rick.mp4', 18523847, 368477, 1852, '2024-03-22 14:37:09+00'::timestamptz),
        ('mr.beast@example.com', 'Survive 30 Days Stranded With Your Ex, Win $250,000', E'╔═╦╗╔╦╗╔═╦═╦╦╦╦╗╔═╗\n║╚╣║║║╚╣╚╣╔╣╔╣║╚╣═╣\n╠╗║╚╝║║╠╗║╚╣║║║║║═╣\n╚═╩══╩═╩═╩═╩╝╚╩═╩═╝\n\nFor any questions or inquiries regarding this video, please reach out to chucky@mrbeastbusiness.com', 1803, 'https://i.ytimg.com/vi/AoN1K4c7VKE/hq720.jpg?sqp=-oaymwEcCNAFEJQDSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLBWvhQfFSlknfGj4Fvqhkl7l9JiiA', 'https://de8x082p75eq8.cloudfront.net/previews/beast-island.webp', 'https://de8x082p75eq8.cloudfront.net/videos/beast-island.mp4', 15238479, 313046, 1356, '2026-03-09 17:04:28+00'::timestamptz),
        ('mr.beast@example.com', '$456,000 Squid Game In Real Life!', E'╔═╦╗╔╦╗╔═╦═╦╦╦╦╗╔═╗\n║╚╣║║║╚╣╚╣╔╣╔╣║╚╣═╣\n╠╗║╚╝║║╠╗║╚╣║║║║║═╣\n╚═╩══╩═╩═╩═╩╝╚╩═╩═╝\n\nFor any questions or inquiries regarding this video, please reach out to chucky@mrbeastbusiness.com', 1541, 'https://i.ytimg.com/vi/0e3GPea1Tyg/hq720.jpg?sqp=-oaymwEcCNAFEJQDSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLBS2AvEddXCN6YbcE6YvKjKAuxPSQ', 'https://de8x082p75eq8.cloudfront.net/previews/beast-squid.webp', 'https://de8x082p75eq8.cloudfront.net/videos/beast-squid.mp4', 912924815, 3845483, 19988, '2022-09-27 11:51:03+00'::timestamptz),
        ('pewdiepie@example.com', 'WHY DID I PLAY THIS?! ((((((So Scary!))))))', E'Official ƿ૯ωძɿ૯ƿɿ૯\n\n⚙️ My Setup (affiliate links)⚙️\n🖥️  PC: https://howl.me/clHcdRoIePi\n🪑 Chair: https://tinyurl.com/clutchpdp\n⌨️ 🖱️ Keyboard & Mouse: https://tinyurl.com/ghostpdp', 2100, 'https://i.ytimg.com/vi/14Rd_h9V4tQ/hq720.jpg?sqp=-oaymwEcCNACELwBSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLAXkVO-aJdhhRpkif2lxJhtMRO5SQ', 'https://de8x082p75eq8.cloudfront.net/previews/pew-scary.webp', 'https://de8x082p75eq8.cloudfront.net/videos/pew-scary.mp4', 1896342, 171923, 819, '2024-11-03 22:47:35+00'::timestamptz),
        ('pewdiepie@example.com', 'I Trained My Own AI... It beat ChatGPT', E'Official ƿ૯ωძɿ૯ƿɿ૯\n\n⚙️ My Setup (affiliate links)⚙️\n🖥️  PC: https://howl.me/clHcdRoIePi\n🪑 Chair: https://tinyurl.com/clutchpdp\n⌨️ 🖱️ Keyboard & Mouse: https://tinyurl.com/ghostpdp', 1535, 'https://i.ytimg.com/vi/aV4j5pXLP-I/hq720.jpg?sqp=-oaymwEcCNACELwBSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLCw5Wg5Tuyfa6VMewRTBd2yfEmPcQ', 'https://de8x082p75eq8.cloudfront.net/previews/pew-ai.webp', 'https://de8x082p75eq8.cloudfront.net/videos/pew-ai.mp4', 2314566, 14423, 75, '2026-02-22 13:29:41+00'::timestamptz),
        ('pewdiepie@example.com', 'I brought the boy to his homeland', E'Official ƿ૯ωძɿ૯ƿɿ૯\n\n⚙️ My Setup (affiliate links)⚙️\n🖥️  PC: https://howl.me/clHcdRoIePi\n🪑 Chair: https://tinyurl.com/clutchpdp\n⌨️ 🖱️ Keyboard & Mouse: https://tinyurl.com/ghostpdp', 1769, 'https://i.ytimg.com/vi/TwtjzPZj2XY/hq720.jpg?sqp=-oaymwEcCNACELwBSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLCUjH2-dzI7OpllhQYg7z8fn7ZqdA', 'https://de8x082p75eq8.cloudfront.net/previews/pew-boy.webp', 'https://de8x082p75eq8.cloudfront.net/videos/pew-boy.mp4', 5117866, 34423, 124, '2025-03-21 13:29:41+00'::timestamptz),
        ('low.level@example.com', 'The C Programming Language is Over 50 Years Old, So Today I Learned Rust', E'🏫 MY COURSES\nSign-up for my FREE 3-Day C Course: https://lowlevel.academy\n\n🔥COME HANG OUT\nCheck out my other stuff: https://lowlevel.tv', 494, 'https://i.ytimg.com/vi/nxpKv0QWs-o/hq720.jpg?sqp=-oaymwEcCNACELwBSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLDZF3m1X6XdY8AjixBnSpJhE5MQng', 'https://de8x082p75eq8.cloudfront.net/previews/low-rust-future.webp', 'https://de8x082p75eq8.cloudfront.net/videos/low-rust.mp4', 11287432, 225748, 1129, '2022-07-05 08:15:27+00'::timestamptz),
        ('low.level@example.com', 'how can HACKERS use Rust for EVIL?? (the future of malware)', E'🏫 MY COURSES\nSign-up for my FREE 3-Day C Course: https://lowlevel.academy\n\n🔥COME HANG OUT\nCheck out my other stuff: https://lowlevel.tv', 237, 'https://i.ytimg.com/vi/11raTwzQVn4/hq720.jpg?sqp=-oaymwEcCNACELwBSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLCEpR2P70EaqTYiNEHo-0GcB1K4Mw', 'https://de8x082p75eq8.cloudfront.net/previews/low-hacker.webp', 'https://de8x082p75eq8.cloudfront.net/videos/low-hacker.mp4', 683492, 13674, 62, '2025-09-30 19:56:14+00'::timestamptz),
        ('low.level@example.com', 'my new wife', E'🏫 MY COURSES\nSign-up for my FREE 3-Day C Course: https://lowlevel.academy\n\n🔥COME HANG OUT\nCheck out my other stuff: https://lowlevel.tv', 265, 'https://i.ytimg.com/vi/UdE8_V05BI8/hq720.jpg?sqp=-oaymwEcCNACELwBSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLC3da7jOCRzF76cib0t4DUOCeSfRg', 'https://de8x082p75eq8.cloudfront.net/previews/low-wife.webp', 'https://de8x082p75eq8.cloudfront.net/videos/low-wife.mp4', 2103492, 86734, 254, '2024-09-30 19:56:14+00'::timestamptz),
        ('low.level@example.com', 'dude wtf', E'🏫 MY COURSES\nSign-up for my FREE 3-Day C Course: https://lowlevel.academy\n\n🔥COME HANG OUT\nCheck out my other stuff: https://lowlevel.tv', 567, 'https://i.ytimg.com/vi/q2WCggEGzFA/hq720.jpg?sqp=-oaymwEcCNACELwBSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLBHShpNVH4oGUbY3pL7RfVlCQtQOQ', 'https://de8x082p75eq8.cloudfront.net/previews/low-dellhack.webp', 'https://de8x082p75eq8.cloudfront.net/videos/low-dell.mp4', 183492, 7662, 2, '2025-09-30 19:56:14+00'::timestamptz),
        ('fireship@example.com', 'Kubernetes Explained in 100 Seconds', 'Learn the basics of Kubernetes and how it''s used to scale containers to massive workloads in the in cloud, in 100 seconds. https://fireship.io/tags/docker/', 126, 'https://i.ytimg.com/vi/PziYflu8cB8/hq720.jpg?sqp=-oaymwEcCNAFEJQDSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLCK4XQkLsEVhgHH4g3WRhDpgvwA4g', 'https://de8x082p75eq8.cloudfront.net/previews/fs-kube.webp', 'https://de8x082p75eq8.cloudfront.net/videos/fs-kube.mp4', 9187634, 183652, 918, '2025-10-18 10:22:59+00'::timestamptz),
        ('fireship@example.com', 'Linux in 100 Seconds', 'Linux is a free and open-source operating system that powers many of the world''s computer systems.', 161, 'https://i.ytimg.com/vi/rrB13utjYV4/hq720.jpg?sqp=-oaymwEcCNAFEJQDSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLB0DiOl5rPN55z4KSBYFJdUt__I2g', 'https://de8x082p75eq8.cloudfront.net/previews/fs-linux.webp', 'https://de8x082p75eq8.cloudfront.net/videos/fs-linux.mp4', 1124718, 20456, 101, '2023-12-07 07:44:06+00'::timestamptz),
        ('netflix@example.com', '「スティール・ボール・ラン ジョジョの奇妙な冒険」｜予告編｜Netflix Japan', E'「スティール・ボール・ラン ジョジョの奇妙な冒険」\nNetflixにて2026年3月19日（木）より独占先行配信決定', 130, 'https://i.ytimg.com/vi/b51C8AbRDGU/hq720.jpg?sqp=-oaymwEcCNAFEJQDSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLD1hCbk_9lcEniiXd9H6meAIIScww', 'https://de8x082p75eq8.cloudfront.net/previews/netflix-jojo.webp', 'https://de8x082p75eq8.cloudfront.net/videos/netflix-jojo.mp4', 873456, 1789, 8, '2026-03-08 15:38:22+00'::timestamptz),
        ('netflix@example.com', '意識は遅れてやってくる| 刃牙道 | Netflix Japan', E'超超高度な意識の読み合い。\n\nNetflix公式SNS:\n➡️X:   / netflixjp\n➡️TIK TOK:   / netflixjapan\n➡️INSTAGRAM:   / netflixjp\n➡️FACEBOOK:   / netflixjp\n➡️ANIME X:   / netflixjp_anime', 69, 'https://i.ytimg.com/vi/UalvUZiyQfw/hq720.jpg?sqp=-oaymwEcCNACELwBSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLCAvugzx8KJwgATbg3XLiT5PM3Qtg', 'https://de8x082p75eq8.cloudfront.net/previews/netflix-baki.webp', 'https://de8x082p75eq8.cloudfront.net/videos/netflix-baki.mp4', 9924, 84, 3, '2024-10-29 12:07:48+00'::timestamptz),
        ('squeezie@example.com', 'Ce milliardaire a tout perdu à cause d’un caprice...', 'Téléchargez REVOLUT gratuitement avec ce lien https://get.revolut.com/z4lF/SqUeeZie\nVous pourrez obtenir 20 € et participer au tirage au sort pour gagner 10 000 € (voir conditions générales) (collaboration commerciale)', 1630, 'https://i.ytimg.com/vi/hV_2UC_6bTU/hq720.jpg?sqp=-oaymwEcCNACELwBSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLAx5sDNIUZsG4wQYXfgK8BMq68HtQ', 'https://de8x082p75eq8.cloudfront.net/previews/squeez-millia.webp', 'https://de8x082p75eq8.cloudfront.net/videos/squeez-milliard.mp4', 5628937, 105186, 152, '2025-03-16 08:32:47+00'::timestamptz),
        ('squeezie@example.com', 'QUI EST LE MEURTRIER ? (ft Inoxtag, Seb, Maghla, Michou, Gotaga, GMK, Terra, Tatiana, Freddy G)', 'Téléchargez REVOLUT gratuitement avec ce lien https://get.revolut.com/z4lF/SqUeeZie\nVous pourrez obtenir 20 € et participer au tirage au sort pour gagner 10 000 € (voir conditions générales) (collaboration commerciale)', 4142, 'https://i.ytimg.com/vi/RG6DsvgW84o/hq720.jpg?sqp=-oaymwEcCNACELwBSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLAwxLbTe8pspb4_8JUV0Hdx0O9BWw', 'https://de8x082p75eq8.cloudfront.net/previews/squeez-murder.webp', 'https://de8x082p75eq8.cloudfront.net/videos/squeez-murder.mp4', 11082341, 169451, 8366, '2024-05-27 20:14:31+00'::timestamptz)
) AS v(uploader_email, title, description, duration_seconds, thumbnail_url, preview_url, video_url, view_count, like_count, dislike_count, created_at)
	ON u.email = v.uploader_email
WHERE NOT EXISTS (
	SELECT 1
	FROM videos existing
	WHERE existing.user_id = u.id
	  AND existing.title = v.title
);

-- =========================
-- Channel subscriber counts mock data
-- =========================

INSERT INTO channels (user_id, subscriber_count, description)
SELECT u.id, c.subscriber_count, c.description
FROM users u
JOIN (
	VALUES
		('rick.astley@example.com', 314595::bigint, 'Rick Astley''s official channel'),
		('mr.beast@example.com', 385458774::bigint, 'Mr Beast''s official channel'),
		('pewdiepie@example.com', 114587633::bigint, 'PewDiePie''s official channel'),
		('fireship@example.com', 3650447::bigint, 'Fireship''s official channel'),
		('netflix@example.com', 1784852::bigint, 'Netflix Japan''s official channel'),
		('squeezie@example.com', 19201548::bigint, 'Squeezie''s official channel'),
		('low.level@example.com', 546::bigint, 'Low Level''s official channel'),
		('ted.ed@example.com', 21400417::bigint, 'TED-Ed''s official channel')
) AS c(email, subscriber_count, description)
	ON u.email = c.email;

-- =========================
-- Comments mock data
-- =========================

INSERT INTO video_comments (video_id, user_id, content, like_count, created_at, updated_at)
SELECT 
	v.id as video_id,
	u.id as user_id,
	comments.content,
	comments.like_count,
	now() - (floor(random() * 90)::int || ' days')::interval as created_at,
	now() as updated_at
FROM videos v
CROSS JOIN (
	SELECT id FROM users 
	WHERE email IN (
		'rick.astley@example.com', 'mr.beast@example.com', 'pewdiepie@example.com', 
		'squeezie@example.com', 'low.level@example.com', 'fireship@example.com',
		'alex.chen@example.com', 'sarah.w@example.com', 'james.r@example.com',
		'emma.t@example.com', 'michael.k@example.com', 'lisa.a@example.com',
		'david.j@example.com', 'maria.g@example.com', 'chris.l@example.com'
	)
) u
CROSS JOIN (
	VALUES
		('This is a classic! Never gets old 😄'::text, 342::int),
		('Still better than modern music tbh', 218),
		('The music video quality is insane for 1987', 157),
		('Rick Astley is a legend, change my mind', 285),
		('I got Rick rolled again...', 423),
		('This song hits different in 2026', 112),
		('Can we just appreciate the lyrics?', 89),
		('The 80s were peak music era', 256),
		('Absolute masterpiece', 334),
		('Never getting tired of this', 198),
		('Vocal performance is incredible', 267),
		('This is timeless art', 145),
		('This is absolutely insane 😭', 1240),
		('MrBeast never disappoints with content', 856),
		('I could never survive this lol', 634),
		('$250k is not enough for this challenge', 512),
		('The production value is unreal', 389),
		('Who else would actually do this for money?', 267),
		('That ending was unexpected 🔥', 445),
		('MrBeast really said "let''s make the most insane video ever"', 198),
		('Props to the team for executing this', 321),
		('I''m exhausted just watching this', 156),
		('This is the best content creator on YouTube', 287),
		('Quality is always consistent with MrBeast', 203),
		('This is the most expensive video ever 💸', 2156),
		('The cinematography is cinema quality', 1834),
		('I genuinely felt the tension', 1562),
		('MrBeast really recreated the whole show', 1289),
		('This deserves way more views', 856),
		('The editing in this video is insane', 743),
		('When MrBeast dropped this I rewatched it 5 times', 612),
		('The production team deserves an award', 489),
		('This is what unlimited budget looks like', 378),
		('The winner was so happy lmao', 267),
		('Every MrBeast video is better than the last', 334),
		('This concept was brilliant', 245),
		('Honestly one of his best videos', 412),
		('Felix''s reaction was priceless 😂', 512),
		('That scare moment was hilarious', 387),
		('I jumped from my seat watching this', 234),
		('Classic PewDiePie horror game session', 156),
		('PewDiePie never fails to entertain', 289),
		('His screams are eternal', 201),
		('Wait, Felix built an AI? That''s actually impressive', 834),
		('This is becoming a tech channel lol', 612),
		('The future is here and it''s PewDiePie', 445),
		('This actually looks legit', 289),
		('I need Felix to explain the code', 167),
		('Amazing how versatile Felix is', 356),
		('This AI is actually crazy smart', 278),
		('This wholesome content hit different 🥺', 678),
		('The emotion in this video is real', 523),
		('Felix''s content has evolved so much', 412),
		('The cinematography is beautiful', 334),
		('This made me smile the whole time', 287),
		('Felix is such a good person', 445),
		('This is heartwarming content', 356),
		('Finally someone explaining Rust properly', 445),
		('Rust is the future of system programming', 378),
		('This comparison blew my mind', 256),
		('Great explanation for beginners', 189),
		('C is still relevant though', 312),
		('Love Low Level Academy content', 234),
		('This is actually terrifying 😬', 567),
		('Low Level always brings the heat', 423),
		('The security implications are wild', 301),
		('This makes me want to learn Rust', 245),
		('Congratulations! 🎉', 312),
		('Happy for you man', 245),
		('Love is in the air', 178),
		('So wholesome 💕', 267),
		('lmaoooo what happened here', 423),
		('That was absolutely unhinged', 312),
		('I lost it 💀', 234),
		('This video is chaotic', 189),
		('Finally understand K8s thanks to this', 678),
		('Fireship makes everything so simple', 534),
		('Best technical explanation ever', 401),
		('Sharing this with my whole team', 289),
		('Fireship is the GOAT', 445),
		('So clear and concise', 356),
		('Linux is life', 456),
		('Finally an OS explanation that makes sense', 334),
		('I''m switching to Linux after watching this', 245),
		('Best 2 minutes of education', 378),
		('ジョジョの実写化が来るとは！', 523),
		('This looks INSANE!', 412),
		('Can''t wait for March 19!', 334),
		('The production value is incredible', 267),
		('Steel Ball Run is my favorite part', 289),
		('刃牙いつ見ても面白い', 289),
		('The animation is so smooth', 201),
		('More Baki content please!', 156),
		('This anime is insane', 223),
		('C''est fou ce qui lui est arrivé 😂', 712),
		('Squeezie fait toujours du banger', 534),
		('La fin était incroyable', 423),
		('Contenu de qualité comme d''hab', 312),
		('Ses vidéos sont toujours cringe 💯', 267),
		('Qu''il est drôle ce Squeezie', 445),
		('Le lineup est incroyable', 856),
		('Vidéo du siècle honnêtement', 712),
		('J''ai pas pu arrêter de regarder', 589),
		('Le twist final était fou', 467),
		('Squeezie know how to make entertaining content', 334),
		('Tous les créateurs français ensemble c''est magique', 289),
		('Absolument génial ce concept', 378)
) AS comments(content, like_count)
WHERE random() < 0.05
ON CONFLICT DO NOTHING;

-- Add comment likes from various users (15% chance per user-comment pair)
INSERT INTO comment_likes (comment_id, user_id)
SELECT c.id, u.id
FROM video_comments c
CROSS JOIN users u
WHERE u.id != c.user_id
  AND random() < 0.15
ON CONFLICT DO NOTHING;

-- =========================
-- End of migration
-- =========================
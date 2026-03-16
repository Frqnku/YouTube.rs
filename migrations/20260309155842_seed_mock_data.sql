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
	('TED-Ed', 'ted.ed@example.com', 'https://www.giantfreakinrobot.com/wp-content/uploads/2022/08/rick-astley.jpg')
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
-- End of migration
-- =========================
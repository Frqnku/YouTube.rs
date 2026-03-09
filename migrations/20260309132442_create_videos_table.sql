-- Migration: Create videos table

CREATE TABLE videos (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    title VARCHAR(200) NOT NULL,
    description VARCHAR(280) NOT NULL,

    duration_seconds INT NOT NULL,

    thumbnail_url TEXT NOT NULL,
    video_url TEXT NOT NULL,

    view_count BIGINT NOT NULL DEFAULT 0,
    like_count BIGINT NOT NULL DEFAULT 0,
    dislike_count BIGINT NOT NULL DEFAULT 0,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- =========================
-- Indexes for performance optimization
-- =========================

-- Fast lookup for a channel's videos
CREATE INDEX idx_videos_user_id
ON videos (user_id);

-- Fast lookup for latest uploads
CREATE INDEX idx_videos_created_at
ON videos (created_at DESC);

-- Fast lookup for videos by title (for search)
CREATE INDEX idx_videos_title
ON videos USING gin (to_tsvector('english', title));

-- =========================
-- Trigger to update updated_at on modification
-- =========================
CREATE TRIGGER trg_videos_updated_at
BEFORE UPDATE ON videos
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- =========================
-- End of migration
-- =========================
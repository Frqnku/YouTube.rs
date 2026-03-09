-- Migration: Create video_views table

CREATE TABLE video_views (
    id BIGSERIAL PRIMARY KEY,
    video_id UUID NOT NULL REFERENCES videos(id) ON DELETE CASCADE,

    user_id UUID REFERENCES users(id),
    ip_address INET,

    watched_seconds INT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- =========================
-- Indexes for performance optimization
-- =========================

-- Fast lookup for views of a video
CREATE INDEX idx_video_views_video_id
ON video_views (video_id);

-- Fast lookup for a user's watch history
CREATE INDEX idx_video_views_user_id_created_at
ON video_views (user_id, created_at DESC);

-- Fast lookup for recent video analytics
CREATE INDEX idx_video_views_video_id_created_at
ON video_views (video_id, created_at DESC);

-- =========================
-- Trigger to update updated_at on modification
-- =========================
CREATE TRIGGER trg_video_views_updated_at
BEFORE UPDATE ON video_views
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- =========================
-- End of migration
-- =========================
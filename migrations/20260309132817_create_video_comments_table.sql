-- Migration: Create video_comments table

CREATE TABLE video_comments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    video_id UUID NOT NULL REFERENCES videos(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    parent_id UUID REFERENCES video_comments(id) ON DELETE CASCADE,

    content VARCHAR(280) NOT NULL,

    like_count INT DEFAULT 0,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- =========================
-- Indexes for performance optimization
-- =========================

-- Fast lookup for comments of a video ordered by recency
CREATE INDEX idx_video_comments_video_id_created_at
ON video_comments (video_id, created_at DESC);

-- Fast lookup for comments by user
CREATE INDEX idx_video_comments_user_id
ON video_comments (user_id);

-- Fast lookup for threaded replies
CREATE INDEX idx_video_comments_parent_id
ON video_comments (parent_id);

-- Fast lookup for popular comments
CREATE INDEX idx_video_comments_like_count
ON video_comments (like_count DESC);

-- =========================
-- Trigger to update updated_at on modification
-- =========================
CREATE TRIGGER trg_video_comments_updated_at
BEFORE UPDATE ON video_comments
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- =========================
-- End of migration
-- =========================
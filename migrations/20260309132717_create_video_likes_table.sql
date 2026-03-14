-- Migration: Create video_reactions table

CREATE TABLE video_reactions (
    video_id UUID REFERENCES videos(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,

    is_liked BOOLEAN NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    PRIMARY KEY(video_id, user_id)
);

-- =========================
-- Indexes for performance optimization
-- =========================

-- Fast lookup for a user's liked/disliked videos
CREATE INDEX idx_video_reactions_user_id_is_liked
ON video_reactions (user_id, is_liked);

-- Fast lookup for per-video reaction stats
CREATE INDEX idx_video_reactions_video_id_is_liked
ON video_reactions (video_id, is_liked);

-- =========================
-- Trigger to update updated_at on modification
-- =========================
CREATE TRIGGER trg_video_reactions_updated_at
BEFORE UPDATE ON video_reactions
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- =========================
-- End of migration
-- =========================
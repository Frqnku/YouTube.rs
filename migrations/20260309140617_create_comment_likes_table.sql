-- Migration: Create comment_likes table

CREATE TABLE comment_likes (
    comment_id UUID REFERENCES video_comments(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    PRIMARY KEY(comment_id, user_id)
);

-- =========================
-- Indexes for performance optimization
-- =========================

-- Fast lookup for a user's liked comments
CREATE INDEX idx_comment_likes_user_id
ON comment_likes (user_id);

-- =========================
-- Trigger to update updated_at on modification
-- =========================
CREATE TRIGGER trg_comment_likes_updated_at
BEFORE UPDATE ON comment_likes
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- =========================
-- End of migration
-- =========================
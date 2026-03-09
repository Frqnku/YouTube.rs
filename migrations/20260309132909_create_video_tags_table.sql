-- Migration: Create video_tags and tags tables

CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
    name VARCHAR(32) UNIQUE NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE video_tags (
    video_id UUID REFERENCES videos(id) ON DELETE CASCADE,
    tag_id INT REFERENCES tags(id) ON DELETE CASCADE,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    PRIMARY KEY(video_id, tag_id)
);

-- =========================
-- Indexes for performance optimization
-- =========================

-- Fast lookup for videos by tag
CREATE INDEX idx_video_tags_tag_id
ON video_tags (tag_id);

-- Fast lookup for recent tag usage
CREATE INDEX idx_video_tags_created_at
ON video_tags (created_at DESC);

-- =========================
-- Trigger to update updated_at on modification
-- =========================
CREATE TRIGGER trg_tags_updated_at
BEFORE UPDATE ON tags
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER trg_video_tags_updated_at
BEFORE UPDATE ON video_tags
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- =========================
-- End of migration
-- =========================
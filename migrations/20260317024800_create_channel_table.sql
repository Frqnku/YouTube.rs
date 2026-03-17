-- Migration: Create channels table

CREATE TABLE channels (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- One channel per user
    user_id UUID NOT NULL
        REFERENCES users(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT channels_user_id_unique UNIQUE (user_id),

    -- Channel information
    description TEXT,
    subscriber_count BIGINT NOT NULL DEFAULT 0,
    video_count BIGINT NOT NULL DEFAULT 0,
    CONSTRAINT channels_subscriber_count_non_negative CHECK (subscriber_count >= 0),
    CONSTRAINT channels_video_count_non_negative CHECK (video_count >= 0),

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- =========================
-- Indexes for performance optimization
-- =========================

CREATE INDEX idx_channels_user_id
ON channels (user_id);

CREATE INDEX idx_channels_subscriber_count_desc
ON channels (subscriber_count DESC);

-- =========================
-- Trigger to update updated_at on modification
-- =========================

CREATE TRIGGER trg_channels_updated_at
BEFORE UPDATE ON channels
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- =========================
-- End of migration
-- =========================
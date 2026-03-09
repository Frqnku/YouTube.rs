-- Migration: Create subscriptions table

CREATE TABLE subscriptions (
    subscriber_id UUID REFERENCES users(id) ON DELETE CASCADE,
    channel_id UUID REFERENCES users(id) ON DELETE CASCADE,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    PRIMARY KEY(subscriber_id, channel_id)
);

-- =========================
-- Indexes for performance optimization
-- =========================

-- Fast lookup for subscribers of a channel
CREATE INDEX idx_subscriptions_channel_id
ON subscriptions (channel_id);

-- Fast lookup for recent subscriptions
CREATE INDEX idx_subscriptions_created_at
ON subscriptions (created_at DESC);

-- =========================
-- Trigger to update updated_at on modification
-- =========================
CREATE TRIGGER trg_subscriptions_updated_at
BEFORE UPDATE ON subscriptions
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- =========================
-- End of migration
-- =========================
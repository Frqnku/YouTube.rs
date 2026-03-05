-- Migration: Create user_oauth_providers table

CREATE TABLE user_oauth_providers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Link to users
    user_id UUID NOT NULL
        REFERENCES users(id)
        ON DELETE CASCADE,

    -- OAuth provider
    provider VARCHAR(30) NOT NULL,
    CONSTRAINT user_oauth_providers_unique UNIQUE (user_id, provider),

    -- Provider user ID (unique per provider)
    provider_user_id VARCHAR(100) NOT NULL,
    CONSTRAINT user_oauth_providers_provider_user_id_unique UNIQUE (provider, provider_user_id),

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- =========================
-- Indexes
-- =========================

-- Lookup by provider_user_id for login
CREATE INDEX idx_user_oauth_providers_provider_user_id
ON user_oauth_providers (provider_user_id);

-- =========================
-- Trigger to auto-update updated_at
-- =========================
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at := now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_user_oauth_providers_updated_at
BEFORE UPDATE ON user_oauth_providers
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- =========================
-- End of migration
-- =========================
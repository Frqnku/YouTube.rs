-- Migration: Create users table

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- First and last name with length limits
    name VARCHAR(50) NOT NULL,

    -- Email with format check and unique constraint
    email VARCHAR(255) NOT NULL,
    CONSTRAINT users_email_unique UNIQUE (email),
    CONSTRAINT users_email_format CHECK (
        email ~* '^[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}$'
    ),

    -- URLs for profile picture
    profile_picture TEXT NULL,
    CONSTRAINT users_profile_picture_format CHECK (
        profile_picture IS NULL OR profile_picture ~* '^(https?://)'
    ),

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- =========================
-- Indexes for performance optimization
-- =========================

-- Fast lookup by email
CREATE INDEX idx_users_email
ON users (email);

-- =========================
-- Trigger to update updated_at on modification
-- =========================
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at := now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_users_updated_at
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- =========================
-- End of migration
-- =========================
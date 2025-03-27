-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email TEXT NOT NULL UNIQUE,
    firstname TEXT NOT NULL,
    lastname TEXT NOT NULL,
    twofa_code TEXT NOT NULL,
    is_2fa_activated BOOLEAN NOT NULL,
    login TEXT NOT NULL UNIQUE,
    roles TEXT[] NOT NULL,
    password TEXT NOT NULL,
    full_text_search TEXT NOT NULL,
    kyber_secret_key TEXT NOT NULL,
    kyber_public_key TEXT NOT NULL,
    iv TEXT NOT NULL,
    is_deleted BOOLEAN NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE,
    deleted_at TIMESTAMP WITH TIME ZONE,
    created_by_id UUID,
    updated_by_id UUID,
    deleted_by_id UUID,
    refresh_token TEXT
);
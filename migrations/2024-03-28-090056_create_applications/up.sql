-- Your SQL goes here
CREATE TABLE applications (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    contact_email TEXT NOT NULL,
    is_system BOOLEAN NOT NULL DEFAULT false,
    keys_number INTEGER NOT NULL DEFAULT 0,
    users_number INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    is_deleted BOOLEAN NOT NULL DEFAULT false,
    updated_at TIMESTAMP WITH TIME ZONE,
    deleted_at TIMESTAMP WITH TIME ZONE,
    created_by_id INTEGER,
    updated_by_id INTEGER,
    deleted_by_id INTEGER
);

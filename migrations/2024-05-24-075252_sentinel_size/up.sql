-- Your SQL goes here
ALTER TABLE sentinels ADD COLUMN key_size integer NOT NULL DEFAULT 256 CHECK (key_size IN (128, 256));

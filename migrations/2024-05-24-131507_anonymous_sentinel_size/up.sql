-- Your SQL goes here
ALTER TABLE anonymous_sentinels ADD COLUMN key_size integer NOT NULL DEFAULT 768 CHECK (key_size IN (512, 768, 1024));

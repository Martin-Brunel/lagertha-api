-- This file should undo anything in `up.sql`
ALTER TABLE revoked_tokens ADD COLUMN user_agent TEXT NOT NULL;

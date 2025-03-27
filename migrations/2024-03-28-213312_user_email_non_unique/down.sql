-- This file should undo anything in `up.sql`
ALTER TABLE users ADD CONSTRAINT users_email_key UNIQUE(email);

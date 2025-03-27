-- This file should undo anything in `up.sql`
ALTER TABLE users ADD CONSTRAINT users_login_key UNIQUE(login);

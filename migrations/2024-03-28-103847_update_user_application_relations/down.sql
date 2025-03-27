-- This file should undo anything in `up.sql`
ALTER TABLE users
  DROP CONSTRAINT IF EXISTS fk_user_application,

ALTER TABLE users DROP COLUMN application;

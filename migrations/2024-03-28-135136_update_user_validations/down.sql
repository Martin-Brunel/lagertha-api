-- This file should undo anything in `up.sql`
ALTER TABLE users DROP COLUMN restricted_ip ;
ALTER TABLE users DROP COLUMN is_validated ;
ALTER TABLE users DROP COLUMN validation_code ;
ALTER TABLE users DROP COLUMN validation_tries;
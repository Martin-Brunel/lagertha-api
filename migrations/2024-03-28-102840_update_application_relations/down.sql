-- This file should undo anything in `up.sql`
ALTER TABLE applications
  DROP CONSTRAINT IF EXISTS fk_created_by,
  DROP CONSTRAINT IF EXISTS fk_updated_by,
  DROP CONSTRAINT IF EXISTS fk_deleted_by;

ALTER TABLE applications DROP COLUMN created_by_id;
ALTER TABLE applications DROP COLUMN updated_by_id;
ALTER TABLE applications DROP COLUMN deleted_by_id;

ALTER TABLE applications ADD COLUMN created_by_id INTEGER;
ALTER TABLE applications ADD COLUMN updated_by_id INTEGER;
ALTER TABLE applications ADD COLUMN deleted_by_id INTEGER;
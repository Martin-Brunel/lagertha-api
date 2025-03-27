-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

ALTER TABLE applications DROP COLUMN created_by_id;
ALTER TABLE applications DROP COLUMN updated_by_id;
ALTER TABLE applications DROP COLUMN deleted_by_id;

ALTER TABLE applications ADD COLUMN created_by_id UUID;
ALTER TABLE applications ADD COLUMN updated_by_id UUID;
ALTER TABLE applications ADD COLUMN deleted_by_id UUID;

ALTER TABLE applications
  ADD CONSTRAINT fk_created_by FOREIGN KEY (created_by_id) REFERENCES users(id),
  ADD CONSTRAINT fk_updated_by FOREIGN KEY (updated_by_id) REFERENCES users(id),
  ADD CONSTRAINT fk_deleted_by FOREIGN KEY (deleted_by_id) REFERENCES users(id);
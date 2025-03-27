-- Your SQL goes here
ALTER TABLE users ADD COLUMN application INTEGER;

ALTER TABLE users
  ADD CONSTRAINT fk_user_application FOREIGN KEY (application) REFERENCES applications(id);
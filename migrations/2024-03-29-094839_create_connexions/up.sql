-- Your SQL goes here
CREATE TABLE connexions (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL,
    ip TEXT NOT NULL,
    user_agent TEXT NOT NULL,
    fingerprint TEXT NOT NULL,
    is_deleted BOOLEAN NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE,
    deleted_at TIMESTAMP WITH TIME ZONE,
    created_by_id UUID,
    updated_by_id UUID,
    deleted_by_id UUID
);

ALTER TABLE connexions
  ADD CONSTRAINT fk_connexion_user_id FOREIGN KEY (user_id) REFERENCES users(id),
  ADD CONSTRAINT fk_connexion_created_by FOREIGN KEY (created_by_id) REFERENCES users(id),
  ADD CONSTRAINT fk_connexion_updated_by FOREIGN KEY (updated_by_id) REFERENCES users(id),
  ADD CONSTRAINT fk_connexion_deleted_by FOREIGN KEY (deleted_by_id) REFERENCES users(id);
-- Your SQL goes here
CREATE TABLE revoked_tokens (
    id SERIAL PRIMARY KEY,
    token TEXT NOT NULL,
    user_agent TEXT NOT NULL,
    is_deleted BOOLEAN NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE,
    deleted_at TIMESTAMP WITH TIME ZONE,
    created_by_id UUID,
    updated_by_id UUID,
    deleted_by_id UUID
);

ALTER TABLE revoked_tokens
  ADD CONSTRAINT fk_revoked_created_by FOREIGN KEY (created_by_id) REFERENCES users(id),
  ADD CONSTRAINT fk_revoked_updated_by FOREIGN KEY (updated_by_id) REFERENCES users(id),
  ADD CONSTRAINT fk_revoked_deleted_by FOREIGN KEY (deleted_by_id) REFERENCES users(id);
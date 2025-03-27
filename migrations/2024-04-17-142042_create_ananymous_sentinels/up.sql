-- Your SQL goes here
CREATE TABLE anonymous_sentinels (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    application_id INT NOT NULL,
    iv VARCHAR(255) NOT NULL,
    sum TEXT NOT NULL,
    public_key TEXT NOT NULL,
    is_deleted BOOLEAN NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE,
    deleted_at TIMESTAMP WITH TIME ZONE,
    created_by_id UUID,
    updated_by_id UUID,
    deleted_by_id UUID
);

ALTER TABLE anonymous_sentinels
  ADD CONSTRAINT fk_anonymous_sentinels_application FOREIGN KEY (application_id) REFERENCES applications(id),
  ADD CONSTRAINT fk_anonymous_sentinels_created_by FOREIGN KEY (created_by_id) REFERENCES users(id),
  ADD CONSTRAINT fk_anonymous_sentinels_updated_by FOREIGN KEY (updated_by_id) REFERENCES users(id),
  ADD CONSTRAINT fk_anonymous_sentinels_deleted_by FOREIGN KEY (deleted_by_id) REFERENCES users(id);

  CREATE TABLE x_anonymous_sentinel_cluster (
    id SERIAL PRIMARY KEY,
    anonymous_sentinel_id UUID NOT NULL,
    cluster_id UUID NOT NULL,
    FOREIGN KEY (anonymous_sentinel_id) REFERENCES anonymous_sentinels(id) ON DELETE CASCADE,
    FOREIGN KEY (cluster_id) REFERENCES clusters(id) ON DELETE CASCADE,
    is_deleted BOOLEAN NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE,
    deleted_at TIMESTAMP WITH TIME ZONE,
    created_by_id UUID,
    updated_by_id UUID,
    deleted_by_id UUID
);

ALTER TABLE x_anonymous_sentinel_cluster
  ADD CONSTRAINT fk_x_anonymous_sentinel_cluster_created_by FOREIGN KEY (created_by_id) REFERENCES users(id),
  ADD CONSTRAINT fk_x_anonymous_sentinel_cluster_updated_by FOREIGN KEY (updated_by_id) REFERENCES users(id),
  ADD CONSTRAINT fk_x_anonymous_sentinel_cluster_deleted_by FOREIGN KEY (deleted_by_id) REFERENCES users(id);
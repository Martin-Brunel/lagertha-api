-- Your SQL goes here
CREATE INDEX index_users_on_is_deleted ON users (is_deleted);
CREATE INDEX index_clusters_on_is_deleted ON clusters (is_deleted);
CREATE INDEX index_sentinels_on_is_deleted ON sentinels (is_deleted);
CREATE INDEX index_x_user_cluster_on_is_deleted ON x_user_cluster (is_deleted);
CREATE INDEX index_x_sentinel_cluster_on_is_deleted ON x_sentinel_cluster (is_deleted);
CREATE INDEX index_applications_on_is_deleted ON applications (is_deleted);
CREATE INDEX index_connexions_on_is_deleted ON connexions (is_deleted);
CREATE INDEX index_revoked_token_on_is_deleted ON revoked_tokens (is_deleted);

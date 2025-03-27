-- This file should undo anything in `up.sql`
DROP INDEX index_users_on_is_deleted;
DROP INDEX index_clusters_on_is_deleted;
DROP INDEX index_sentinels_on_is_deleted;
DROP INDEX index_x_user_cluster_on_is_deleted;
DROP INDEX index_x_sentinel_cluster_on_is_deleted;
DROP INDEX index_applications_on_is_deleted;
DROP INDEX index_connexions_on_is_deleted;
DROP INDEX index_revoked_token_on_is_deleted;
// @generated automatically by Diesel CLI.

diesel::table! {
    anonymous_sentinels (id) {
        id -> Uuid,
        application_id -> Int4,
        #[max_length = 255]
        iv -> Varchar,
        sum -> Text,
        public_key -> Text,
        is_deleted -> Bool,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
        created_by_id -> Nullable<Uuid>,
        updated_by_id -> Nullable<Uuid>,
        deleted_by_id -> Nullable<Uuid>,
        key_size -> Int4,
    }
}

diesel::table! {
    applications (id) {
        id -> Int4,
        name -> Text,
        contact_email -> Text,
        is_system -> Bool,
        keys_number -> Int4,
        users_number -> Int4,
        created_at -> Timestamptz,
        is_deleted -> Bool,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
        created_by_id -> Nullable<Uuid>,
        updated_by_id -> Nullable<Uuid>,
        deleted_by_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    clusters (id) {
        id -> Uuid,
        application_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        is_deleted -> Bool,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
        created_by_id -> Nullable<Uuid>,
        updated_by_id -> Nullable<Uuid>,
        deleted_by_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    connexions (id) {
        id -> Int4,
        user_id -> Uuid,
        ip -> Text,
        user_agent -> Text,
        fingerprint -> Text,
        is_deleted -> Bool,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
        created_by_id -> Nullable<Uuid>,
        updated_by_id -> Nullable<Uuid>,
        deleted_by_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    revoked_tokens (id) {
        id -> Int4,
        token -> Text,
        is_deleted -> Bool,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
        created_by_id -> Nullable<Uuid>,
        updated_by_id -> Nullable<Uuid>,
        deleted_by_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    sentinels (id) {
        id -> Uuid,
        application_id -> Int4,
        #[max_length = 255]
        iv -> Varchar,
        sum -> Text,
        is_deleted -> Bool,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
        created_by_id -> Nullable<Uuid>,
        updated_by_id -> Nullable<Uuid>,
        deleted_by_id -> Nullable<Uuid>,
        key_size -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Text,
        firstname -> Text,
        lastname -> Text,
        twofa_code -> Text,
        is_2fa_activated -> Bool,
        login -> Text,
        roles -> Array<Nullable<Text>>,
        password -> Nullable<Text>,
        full_text_search -> Text,
        kyber_secret_key -> Text,
        kyber_public_key -> Text,
        iv -> Text,
        is_deleted -> Bool,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
        created_by_id -> Nullable<Uuid>,
        updated_by_id -> Nullable<Uuid>,
        deleted_by_id -> Nullable<Uuid>,
        refresh_token -> Nullable<Text>,
        application -> Nullable<Int4>,
        restricted_ip -> Array<Nullable<Text>>,
        is_validated -> Bool,
        validation_code -> Nullable<Text>,
        validation_tries -> Int4,
        forget_code_delay -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    x_anonymous_sentinel_cluster (id) {
        id -> Int4,
        anonymous_sentinel_id -> Uuid,
        cluster_id -> Uuid,
        is_deleted -> Bool,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
        created_by_id -> Nullable<Uuid>,
        updated_by_id -> Nullable<Uuid>,
        deleted_by_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    x_sentinel_cluster (id) {
        id -> Int4,
        sentinel_id -> Uuid,
        cluster_id -> Uuid,
        is_deleted -> Bool,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
        created_by_id -> Nullable<Uuid>,
        updated_by_id -> Nullable<Uuid>,
        deleted_by_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    x_user_cluster (id) {
        id -> Int4,
        user_id -> Uuid,
        cluster_id -> Uuid,
        is_deleted -> Bool,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
        created_by_id -> Nullable<Uuid>,
        updated_by_id -> Nullable<Uuid>,
        deleted_by_id -> Nullable<Uuid>,
    }
}

diesel::joinable!(anonymous_sentinels -> applications (application_id));
diesel::joinable!(clusters -> applications (application_id));
diesel::joinable!(sentinels -> applications (application_id));
diesel::joinable!(x_anonymous_sentinel_cluster -> anonymous_sentinels (anonymous_sentinel_id));
diesel::joinable!(x_anonymous_sentinel_cluster -> clusters (cluster_id));
diesel::joinable!(x_sentinel_cluster -> clusters (cluster_id));
diesel::joinable!(x_sentinel_cluster -> sentinels (sentinel_id));
diesel::joinable!(x_user_cluster -> clusters (cluster_id));

diesel::allow_tables_to_appear_in_same_query!(
    anonymous_sentinels,
    applications,
    clusters,
    connexions,
    revoked_tokens,
    sentinels,
    users,
    x_anonymous_sentinel_cluster,
    x_sentinel_cluster,
    x_user_cluster,
);

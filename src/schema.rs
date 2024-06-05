// @generated automatically by Diesel CLI.

diesel::table! {
    _sqlx_migrations (version) {
        version -> Int8,
        description -> Text,
        installed_on -> Timestamptz,
        success -> Bool,
        checksum -> Bytea,
        execution_time -> Int8,
    }
}

diesel::table! {
    post (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        fullname -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        password -> Text,
        birth_place -> Text,
        birth_date -> Date,
        #[max_length = 255]
        gender -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    _sqlx_migrations,
    post,
    posts,
    users,
);
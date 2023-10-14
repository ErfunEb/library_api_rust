// @generated automatically by Diesel CLI.

diesel::table! {
    genres (id) {
        id -> Text,
        title -> Text,
        parent_id -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

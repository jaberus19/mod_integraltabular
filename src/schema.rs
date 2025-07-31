// @generated automatically by Diesel CLI.

diesel::table! {
    EXPRESSIONS (id) {
        id -> Integer,
        expressions_text -> Text,
        result_text -> Nullable<Text>,
        creat_at -> Timestamp,
    }
}

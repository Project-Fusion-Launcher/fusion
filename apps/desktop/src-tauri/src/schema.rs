// @generated automatically by Diesel CLI.

diesel::table! {
    configs (id) {
        id -> Integer,
        itchio_api_key -> Nullable<Text>,
    }
}

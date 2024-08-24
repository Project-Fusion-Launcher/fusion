diesel::table! {
    configs (id) {
        id -> Integer,
        itchio_api_key -> Nullable<Text>,
    }
}

diesel::table! {
    games (id) {
        id -> Text,
        title -> Text,
    }
}

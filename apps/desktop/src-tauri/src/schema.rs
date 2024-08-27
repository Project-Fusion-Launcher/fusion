diesel::table! {
    configs (id) {
        id -> Integer,
        itchio_api_key -> Nullable<Text>,
    }
}

diesel::table! {
    games (id, source) {
        id -> Text,
        source -> Text,
        title -> Text,
        key -> Nullable<Text>,
    }
}

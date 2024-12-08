diesel::table! {
    configs (id) {
        id -> Integer,
        itchio_api_key -> Nullable<Text>,
        legacy_games_token -> Nullable<Text>,
        legacy_games_email -> Nullable<Text>,
    }
}

diesel::table! {
    games (id, source) {
        id -> Text,
        source -> crate::models::game::GameSourceMapping,
        title -> Text,
        key -> Nullable<Text>,
        developer -> Nullable<Text>,
        launch_target -> Nullable<Text>,
        path -> Nullable<Text>,
        version -> Nullable<Text>,
        status -> crate::models::game::GameStatusMapping,
        favorite -> Bool,
        hidden -> Bool,
    }
}

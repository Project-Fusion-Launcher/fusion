diesel::table! {
    configs (id) {
        id -> Integer,
        it_api_key -> Nullable<Text>,
        lg_token -> Nullable<Text>,
        lg_email -> Nullable<Text>,
        eg_refresh_token -> Nullable<Text>,
    }
}

diesel::table! {
    games (id, source) {
        id -> Text,
        source -> crate::models::GameSourceMapping,
        name -> Text,
        sort_name -> Text,
        developer -> Nullable<Text>,
        status -> crate::models::GameStatusMapping,
        favorite -> Bool,
        hidden -> Bool,
        cover_url -> Nullable<Text>,
    }
}

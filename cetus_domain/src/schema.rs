table! {
    profiles (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Text,
        bio -> Text,
        avatar_url -> Text,
        public_repos -> Int4,
        public_gists -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int4,
        gh_username -> Text,
        gh_token -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(profiles -> users (user_id));

allow_tables_to_appear_in_same_query!(profiles, users,);

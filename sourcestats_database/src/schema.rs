table! {
    accounts (user_id) {
        user_id -> Int4,
        email -> Text,
        username -> Varchar,
        password -> Bpchar,
    }
}

table! {
    players (player_id) {
        player_id -> Int8,
        steam_id -> Int8,
    }
}

table! {
    server_keys (key_id) {
        key_id -> Int8,
        key_data -> Bytea,
        server_id -> Int8,
    }
}

table! {
    servers (server_id) {
        server_id -> Int8,
        server_name -> Text,
        server_address -> Bpchar,
        server_website -> Nullable<Text>,
    }
}

joinable!(server_keys -> servers (server_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    players,
    server_keys,
    servers,
);

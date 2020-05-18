table! {
    streams (id) {
        id -> Nullable<Integer>,
        service -> Text,
        channel -> Text,
        path -> Nullable<Text>,
        hidden -> Nullable<Bool>,
        afk -> Nullable<Bool>,
        promoted -> Nullable<Bool>,
        title -> Text,
        thumbnail -> Nullable<Text>,
        live -> Nullable<Bool>,
        viewers -> Nullable<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Text,
        twitch_id -> BigInt,
        name -> Text,
        stream_path -> Text,
        service -> Text,
        channel -> Text,
        last_ip -> Text,
        last_seen -> Timestamp,
        left_chat -> Nullable<Bool>,
        is_banned -> Bool,
        ban_reason -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_admin -> Nullable<Bool>,
    }
}

allow_tables_to_appear_in_same_query!(streams, users,);

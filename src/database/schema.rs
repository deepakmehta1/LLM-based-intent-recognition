use diesel::table;

table! {
    messages (id) {
        id -> Int4,
        role -> Varchar,
        content -> Text,
        created_at -> Timestamp,
    }
}

table! {
    todos (id) {
        id -> Int4,
        title -> Varchar,
        created_at -> Timestamp,
        completed_at -> Nullable<Timestamp>,
    }
}

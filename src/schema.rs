table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    posts_revisions (id) {
        id -> Int4,
        created -> Timestamp,
        post_id -> Int4,
        title -> Varchar,
        body -> Text,
    }
}

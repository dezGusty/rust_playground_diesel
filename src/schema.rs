// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        published -> Integer,
        entry_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    weights (id) {
        id -> Integer,
        weight -> Float,
        measurement_date -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    weights,
);

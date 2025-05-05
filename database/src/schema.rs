// @generated automatically by Diesel CLI.

diesel::table! {
    test (id) {
        id -> Int4,
        name -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

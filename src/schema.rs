// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        body -> Text,
        published -> Bool,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        google_id -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        user_name -> Varchar,
        verified -> Nullable<Bool>,
        #[max_length = 255]
        provider -> Varchar,
        #[max_length = 255]
        photo -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);

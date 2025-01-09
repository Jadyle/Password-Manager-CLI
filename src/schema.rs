// @generated automatically by Diesel CLI.

diesel::table! {
    passwords (id) {
        id -> Integer,
        label -> Text,
        r_password -> Text,
        salt_password -> Text,
        nonce_password -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        user -> Text,
        label_account -> Text,
        salt_account -> Text,
        master_password -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    passwords,
    users,
);

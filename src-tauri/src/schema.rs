// @generated automatically by Diesel CLI.

diesel::table! {
    babies (id) {
        id -> Text,
        name -> Text,
        photo -> Nullable<Text>,
        sex -> Text,
        birthday -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    comments (id) {
        id -> Text,
        note_id -> Text,
        comment -> Nullable<Text>,
        image -> Nullable<Text>,
        file -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    notes (id) {
        id -> Text,
        baby_id -> Text,
        note -> Nullable<Text>,
        image -> Nullable<Text>,
        file -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(comments -> notes (note_id));
diesel::joinable!(notes -> babies (baby_id));

diesel::allow_tables_to_appear_in_same_query!(
    babies,
    comments,
    notes,
);

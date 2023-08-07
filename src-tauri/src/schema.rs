// @generated automatically by Diesel CLI.

diesel::table! {
    babies (id) {
        id -> Integer,
        name -> Text,
        sex -> Text,
        birthday -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    comments (id) {
        id -> Integer,
        note_id -> Integer,
        comment -> Nullable<Text>,
        image -> Nullable<Binary>,
        file -> Nullable<Binary>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    notes (id) {
        id -> Integer,
        baby_id -> Integer,
        note -> Nullable<Text>,
        image -> Nullable<Binary>,
        file -> Nullable<Binary>,
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

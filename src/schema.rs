// @generated automatically by Diesel CLI.

diesel::table! {
    device (id) {
        id -> Integer,
        room -> Integer,
        name -> Text,
        device_type -> Text,
        state -> Bool,
    }
}

diesel::table! {
    house (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    room (id) {
        id -> Integer,
        house -> Integer,
        name -> Text,
    }
}

diesel::joinable!(device -> room (room));
diesel::joinable!(room -> house (house));

diesel::allow_tables_to_appear_in_same_query!(
    device,
    house,
    room,
);

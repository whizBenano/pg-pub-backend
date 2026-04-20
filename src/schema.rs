// @generated automatically by Diesel CLI.

diesel::table! {
    people (id) {
        id -> Uuid,
        name -> Text,
        email -> Text,
        password -> Text,
        verified -> Bool,
    }
}

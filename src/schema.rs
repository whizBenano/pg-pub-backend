// @generated automatically by Diesel CLI.

diesel::table! {
    books (book_id) {
        book_id -> Uuid,
        author_id -> Uuid,
        book_title -> Text,
        content -> Nullable<Text>,
        price -> Float8,
        img_url -> Nullable<Text>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    posts (post_id) {
        post_id -> Uuid,
        author_id -> Uuid,
        content -> Nullable<Text>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    publications (publication_id) {
        publication_id -> Uuid,
        author_id -> Uuid,
        book_title -> Text,
        img_url -> Nullable<Text>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Uuid,
        first_name -> Text,
        last_name -> Text,
        other_names -> Nullable<Text>,
        phone_number -> Nullable<Text>,
        profile_picture_url -> Nullable<Text>,
        email -> Text,
        password -> Text,
        created_at -> Timestamptz,
        verified -> Bool,
    }
}

diesel::joinable!(books -> users (author_id));
diesel::joinable!(posts -> users (author_id));
diesel::joinable!(publications -> users (author_id));

diesel::allow_tables_to_appear_in_same_query!(books, posts, publications, users,);

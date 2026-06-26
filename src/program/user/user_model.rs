use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{Utc, DateTime};
// use super::book::book_model::Book;

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    pub first_name: String,
    pub last_name: String,
    pub other_names: Option<String>,
    pub phone_number: Option<String>,
    pub user_name: Option<String>,
    pub profile_picture_url: Option<String>,
    pub email: String,
    pub password: String,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = crate::program::schema::users)]
pub struct UpdateUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub other_names: Option<String>,
    pub phone_number: Option<String>,
    pub profile_picture_url: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::program::schema::users)]
pub struct User {
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub other_names: Option<String>,
    pub user_name: Option<String>,
    pub phone_number: Option<String>,
    pub profile_picture_url: Option<String>,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub verified: bool,
    // pub books: Option<Vec<Book>>
}

#[derive(Insertable)]
#[diesel(table_name = crate::program::schema::user_photos)]
pub struct NewUserPhoto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub image_data: Vec<u8>,
}

#[derive(Queryable)]
pub struct _UserPhoto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub image_data: Vec<u8>,
    pub uploaded_at: DateTime<Utc>
}

#[derive(Deserialize)]
pub struct LoginData {
    pub email: String,
    pub password: String
}

#[derive(Serialize)]
pub struct Profile {
    pub first_name: String,
    pub last_name: String,
    pub other_names: Option<String>,
    pub phone_number: Option<String>,
    pub profile_picture_url: Option<String>,
    pub email: String,
    // pub books: Option<Vec<Book>>
}

impl User {
    pub fn to_dash(self) -> Profile {
        Profile {
            first_name: self.first_name,
            last_name: self.last_name,
            other_names: self.other_names,
            phone_number: self.phone_number,
            profile_picture_url: self.profile_picture_url,
            email: self.email
        }
    }
}
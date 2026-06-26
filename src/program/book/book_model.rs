use serde::{Deserialize, Serialize};
use uuid::Uuid;
use diesel::prelude::*;
use chrono::{Utc, DateTime};

#[derive(Serialize, Deserialize)]
pub struct _CreateBook {
    pub author_id: Uuid,
    pub book_title: String,
    pub content: String,
    pub price: f64,
    pub synopsis: String,
    pub img_url: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::program::schema::books)]
pub struct Book {
    pub book_id: Uuid,
    pub author_id: Uuid,
    pub book_title: String,
    pub content: Option<String>,
    pub price: f64,
    pub rating: f64,
    pub img_url: Option<String>,
    pub synopsis: String,
    pub created_at: DateTime<Utc>,
}

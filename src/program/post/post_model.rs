use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use diesel::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct _CreatePost {
    pub content: String,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = crate::program::schema::posts)]
pub struct UpdatePost {
    pub content: String,
    pub updated_at: DateTime<Utc>
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::program::schema::posts)]
pub struct Post {
    pub post_id: Uuid,
    pub author_id: Uuid,
    pub thumbnail: Option<String>,
    pub content: String,
    pub sparks: i32,
    pub echoes: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}
use actix_cors::Cors;
use actix_web::http::header;
use diesel::{r2d2::{ConnectionManager, Pool}, prelude::*, PgConnection};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::people)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::people)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub verified: bool
}

#[derive(Serialize)]
pub struct Dashboard {
    pub name: String,
    pub email: String
}

#[derive(Deserialize)]
pub struct LoginData {
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64
}

impl User {
    pub fn to_dash(self) -> Dashboard {
        Dashboard {
            name: self.name,
            email: self.email
        }
    }
}

pub fn cors() -> Cors {
    Cors::default()
        .allowed_origin("http://localhost:5173")
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT])
        .supports_credentials()
}

pub async fn init_pool() -> DbPool {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").map_err(|_| "Unable to read DB URL").unwrap();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager).map_err(|_| "Failed to create a connection pool").unwrap()
}
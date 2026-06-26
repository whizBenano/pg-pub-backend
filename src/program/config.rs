use actix_cors::Cors;
use actix_web::http::header;
use diesel::{PgConnection, r2d2::{ConnectionManager, Pool}};
use serde::{Serialize, Deserialize};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64
}

pub fn cors() -> Cors {
    Cors::default()
        .allowed_origin("http://localhost:3000")
        .allowed_origin("http://127.0.0.1:3000")
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT])
        .supports_credentials()
}

pub async fn init_pool() -> DbPool {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").map_err(|_| "Unable to locate DB URL").unwrap();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager).map_err(|_| "Failed to initialize a DB pool").unwrap()
}
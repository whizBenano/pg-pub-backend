use actix_web::{cookie::{time::Duration, Cookie, SameSite}, web::Data};
use argon2::{password_hash::{rand_core::OsRng, SaltString}, Argon2, PasswordHasher};
use diesel::{r2d2::{ConnectionManager, PooledConnection}, PgConnection};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::{Error, Uuid};
use super::model::{Claims, DbPool};

pub fn conn(db: Data<DbPool>) -> PooledConnection<ConnectionManager<PgConnection>> {
    db.get().map_err(|_| "Unable to get a DB connection").unwrap()
}

pub fn hash_pass(password_input: Option<String>) -> Option<String> {
    match password_input {
        Some(passwd) => {
            let argon2 = Argon2::default();
            let salt = SaltString::generate(OsRng);
        
            Some(argon2.hash_password(passwd.as_bytes(), &salt).unwrap().to_string())
        },
        None => None
    }
}

pub fn create_cookie(user_id: Uuid) -> Cookie<'static> {
    let secret = std::env::var("JWT_SECRET").map_err(|_| "Unable to read JWT secret").unwrap();
    let claims = Claims {
        sub: user_id.to_string(),
        exp: chrono::Utc::now().timestamp() + 300,
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap();

    Cookie::build("token", token)
        .path("/")
        .max_age(Duration::seconds(300))
        .same_site(SameSite::Lax)
        .http_only(true)
        .secure(false)
        .finish()
}

pub fn decode_token(token: &str) -> Result<Uuid, Error> {
    let secret = std::env::var("JWT_SECRET").map_err(|_| "Unable to read JWT secret").unwrap();

    let token_data = decode::<Claims>(
        &token, 
        &DecodingKey::from_secret(secret.as_ref()), 
        &Validation::default()
    )
    .unwrap();

    Uuid::parse_str(&token_data.claims.sub)
}

pub fn clear_cookie() -> Cookie<'static> {
    Cookie::build("token", "")
        .path("/")
        .max_age(Duration::seconds(0))
        .same_site(SameSite::Lax)
        .http_only(true)
        .secure(false)
        .finish()
}
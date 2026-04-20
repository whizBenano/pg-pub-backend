use actix_web::{delete, get, http::header, post, put, web::{Data, Json}, HttpRequest, HttpResponse, Responder};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use diesel::prelude::*;
use super::{model::{DbPool, User, CreateUser, UpdateUser, LoginData}, services::{conn, hash_pass, create_cookie, decode_token, clear_cookie}};
use uuid::Uuid;
use crate::schema::people::dsl::*;

#[get("/users")]
async fn get_users(data: Data<DbPool>) -> impl Responder {
    match people.load::<User>(&mut conn(data)) {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(ex) => HttpResponse::InternalServerError().body(format!("{}", ex))
    }
}

#[post("/register")]
async fn create_user(data: Data<DbPool>, form: Json<CreateUser>) -> impl Responder {
    let new_user = User {
        id: Uuid::new_v4(),
        name: form.name.clone(),
        email: form.email.clone(),
        password: hash_pass(Some(form.password.clone())).unwrap(),
        verified: false
    };

    // match diesel::insert_into(people).values(new_user).execute(&mut conn(data)) {
    //     Ok(_) => HttpResponse::Ok().body("Registration successful"),
    //     Err(ex) => HttpResponse::InternalServerError().body(format!("{}", ex))
    // }

    diesel::insert_into(people).values(new_user)
        .execute(&mut conn(data))
        .map_or_else(
            |_| HttpResponse::InternalServerError().body("Registration failed"),
            |_| HttpResponse::Ok().body("Registration successful")
        )
}

#[post("/login")]
async fn login_user(data: Data<DbPool>, form: Json<LoginData>) -> impl Responder {
    let user = people.filter(email.eq(&form.email)).first::<User>(&mut conn(data));

    match user {
        Ok(user) => {
            let argon2 = Argon2::default();
            let parsed = PasswordHash::new(&user.password).unwrap();

            match argon2.verify_password(form.password.as_bytes(), &parsed) {
                Ok(_) => {
                    HttpResponse::SeeOther()
                        .cookie(create_cookie(user.id))
                        .insert_header((header::LOCATION, "/dashboard"))
                        .finish()
                },
                Err(ex) => HttpResponse::Unauthorized().body(format!("{}", ex))
            }
        },
        Err(ex) => HttpResponse::Unauthorized().body(format!("{}", ex))
    }
}

#[get("/dashboard")]
async fn dashboard(data: Data<DbPool>, req: HttpRequest) -> impl Responder {
    if let Some(cookie) = req.cookie("token") {
        match decode_token(cookie.value()) {
            Ok(user_id) => {
                let user = people.find(user_id).first::<User>(&mut conn(data)).unwrap();
                HttpResponse::Ok().json(user.to_dash())
            },
            Err(_) => HttpResponse::Unauthorized().body("Invalid token")
        }
    } else {
        HttpResponse::Unauthorized().body("You're not logged in!")
    }
}

#[put("/user")]
async fn update_user(data: Data<DbPool>, form: Json<UpdateUser>, req: HttpRequest) -> impl Responder {
    if let Some(cookie) = req.cookie("token") {
        match decode_token(cookie.value()) {
            Ok(user_id) => {
                let updated = UpdateUser {
                    name: form.name.clone(),
                    email: form.email.clone(),
                    password: hash_pass(form.password.clone())
                };

                match diesel::update(people.find(user_id)).set(updated).execute(&mut conn(data)) {
                    Ok(_) => HttpResponse::SeeOther().insert_header((header::LOCATION, "/dashboard")).finish(),
                    Err(ex) => HttpResponse::InternalServerError().body(format!("{}", ex))
                }
            },
            Err(_) => HttpResponse::Unauthorized().body("Invalid token")
        }
    } else {
        HttpResponse::Unauthorized().body("You're not logged in!")
    }
}

#[delete("/user")]
async fn delete_user(data: Data<DbPool>, req: HttpRequest) -> impl Responder {
    if let Some(cookie) = req.cookie("token") {
        match decode_token(cookie.value()) {
            Ok(user_id) => {
                match diesel::delete(people.find(user_id)).execute(&mut conn(data)) {
                    Ok(_) => HttpResponse::SeeOther().cookie(clear_cookie()).insert_header((header::LOCATION, "/people")).finish(),
                    Err(ex) => HttpResponse::InternalServerError().body(format!("{}", ex))
                }
            },
            Err(_) => HttpResponse::Unauthorized().body("Invalid or expired token")
        }
    } else {
        HttpResponse::Unauthorized().body("You're not logged in")
    }
}

#[get("/logout")]
async fn logout_user() -> impl Responder {
    HttpResponse::SeeOther()
        .cookie(clear_cookie())
        .insert_header((header::LOCATION, "/people"))
        .finish()
}
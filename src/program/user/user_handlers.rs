use actix_web::{HttpRequest, HttpResponse, Responder, delete, get, http::header, patch, post, web::{Data, Json, Path}};
// use actix_multipart::Multipart;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use super::{user_model::{CreateUser, UpdateUser, LoginData, User, _UserPhoto}, user_services::{conn, hash_pass, create_cookie, decode_token, clear_cookie}};
use crate::program::{schema::{users::dsl::*, user_photos}, config::DbPool};
use diesel::prelude::*;
use uuid::Uuid;
// use futures_util::StreamExt;
// use tokio::io::AsyncWriteExt;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello from the backend")
}

#[post("/register")]
async fn create_user(data: Data<DbPool>, form: Json<CreateUser>) -> impl Responder {
    let new_user = User {
        user_id: Uuid::new_v4(),
        first_name: form.first_name.clone(),
        last_name: form.last_name.clone(),
        other_names: form.other_names.clone(),
        user_name: form.user_name.clone(),
        phone_number: form.phone_number.clone(),
        profile_picture_url: form.profile_picture_url.clone(),
        email: form.email.clone(),
        password: hash_pass(Some(form.password.clone())).unwrap(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        verified: false
    };

    diesel::insert_into(users).values(new_user).execute(&mut conn(data)).map_or_else(
        |ex| HttpResponse::InternalServerError().body(format!("Registration failed: {}", ex)),
        |_| HttpResponse::Ok().body("Registered successfully")
    )
}

#[post("/login")]
async fn login_user(data: Data<DbPool>, form: Json<LoginData>) -> impl Responder {
    let user = users.filter(email.eq(&form.email)).first::<User>(&mut conn(data));

    match user {
        Ok(user) => {
            let argon2 = Argon2::default();
            let parsed = PasswordHash::new(&user.password).unwrap();

            argon2.verify_password(form.password.as_bytes(), &parsed).map_or_else(
                |ex| HttpResponse::Unauthorized().body(format!("{}", ex)),
                |_| HttpResponse::Ok().cookie(create_cookie(user.user_id)).json("Logged in")
            )
        },
        Err(ex) => HttpResponse::Unauthorized().body(format!("{}", ex))
    }
}

#[get("/dashboard")]
async fn dashboard(data: Data<DbPool>, req: HttpRequest) -> impl Responder {
    if let Some(cookie) = req.cookie("token") {
        decode_token(cookie.value()).map_or_else(
            |_| HttpResponse::Unauthorized().json("Inavalid token"),
            |id| {
                let person = users.find(id).first::<User>(&mut conn(data))
                    .map_err(|_| "User not found").unwrap();
                HttpResponse::Ok().json(person.to_dash())
            }
        )
    } else {
        HttpResponse::Unauthorized().json("You're not logged in")
    }
}

#[get("/profile")]
async fn profile(data: Data<DbPool>, req: HttpRequest) -> impl Responder {
    if let Some(cookie) = req.cookie("token") {
        decode_token(cookie.value()).map_or_else(
            |_| HttpResponse::Unauthorized().json("Inavalid token"),
            |id| {
                let person = users.find(id).first::<User>(&mut conn(data))
                    .map_err(|_| "User not found").unwrap();
                HttpResponse::Ok().json(person.to_dash())
            }
        )
    } else {
        HttpResponse::Unauthorized().json("You're not logged in")
    }
}

#[patch("/user")]
async fn update_user(data: Data<DbPool>, form: Json<UpdateUser>, req: HttpRequest) -> impl Responder {
    if let Some(cookie) = req.cookie("token") {
        decode_token(cookie.value()).map_or_else(
            |_| HttpResponse::Unauthorized().body("You're not logged in"),
            |id| {
                let updated = UpdateUser {
                    first_name: form.first_name.clone(),
                    last_name: form.last_name.clone(),
                    other_names: form.other_names.clone(),
                    phone_number: form.phone_number.clone(),
                    profile_picture_url: form.profile_picture_url.clone(),
                    email: form.email.clone(),
                    password: hash_pass(form.password.clone())
                };

                diesel::update(users.find(id)).set(updated).execute(&mut conn(data)).map_or_else(
                    |ex| HttpResponse::InternalServerError().body(format!("{}", ex)),
                    |_| HttpResponse::SeeOther().insert_header((header::LOCATION, "/dashboard")).finish()
                )
            }
        )
    } else {
        HttpResponse::Unauthorized().body("You're not logged in")
    }
}

#[delete("/user")]
async fn delete_user(data: Data<DbPool>, req: HttpRequest) -> impl Responder {
    if let Some(cookie) = req.cookie("token") {
        decode_token(cookie.value()).map_or_else(
            |_| HttpResponse::Unauthorized().body("Invalid token"),
            |id| {
                diesel::delete(users.find(id)).execute(&mut conn(data)).map_or_else(
                    |ex| HttpResponse::InternalServerError().body(format!("{}", ex)),
                    |_| HttpResponse::SeeOther().cookie(clear_cookie()).insert_header((header::LOCATION, "/users")).finish()
                )
            }
        )
    } else {
        HttpResponse::Unauthorized().body("You're not logged")
    }
}






#[get("/photos/{id}")]
async fn get_photo(photo_id: Path<Uuid>, data: Data<DbPool>) -> impl Responder {
    // let photo = user_photos::table.find(*photo_id).first::<UserPhoto>(&mut conn(data))?;

    // Ok(
    //     HttpResponse::Ok()
    //         .content_type("image/jpeg")
    //         .body(photo.image_data)
    // )

    match user_photos::table.find(*photo_id).first::<_UserPhoto>(&mut conn(data)) {
        Ok(photo) => HttpResponse::Ok().content_type("image/jpeg").body(photo.image_data),
        Err(_) => HttpResponse::NoContent().finish()
    }
}









#[get("/logout")]
async fn logout_user() -> impl Responder {
    HttpResponse::Ok().cookie(clear_cookie()).finish()
}










// #[post("/upload-photo")]
// async fn upload_photo(data: Data<DbPool>, mut payload: Multipart, id: Uuid) -> Result<(), Box<dyn std::error::Error>> {

//     while let Some(field) = payload.next().await {
//         let mut field = field?;

//         let mut image_bytes = Vec::new();

//         while let Some(chunk) = field.next().await {
//             let data = chunk?;
//             image_bytes.extend_from_slice(&data);
//         }

//         let photo = NewUserPhoto {
//             id: Uuid::new_v4(),
//             user_id,
//             image_data: image_bytes,
//         };

//         diesel::insert_into(user_photos::table)
//             .values(&photo)
//             .execute(&mut conn(data))?;

//         break;
//     }

//     Ok(())
// }
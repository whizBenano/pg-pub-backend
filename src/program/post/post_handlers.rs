use actix_web::{HttpRequest, HttpResponse, Responder, delete, get, patch, post, web::{Data, Json, Path}};
use diesel::prelude::*;
use uuid::Uuid;
use super::{post_model::{Post, _CreatePost, UpdatePost}, super::{config::DbPool, user::user_services::{conn, decode_token}}};
use crate::program::schema::posts::dsl::*;

#[get("/posts")]
async fn get_posts(data: Data<DbPool>) -> impl Responder {
    posts.load::<Post>(&mut conn(data)).map_or_else(
        |_| HttpResponse::NoContent().body("No posts yet"),
        |list| HttpResponse::Ok().json(list)
    )
}

#[get("/posts/{id}")]
async fn get_post(data: Data<DbPool>, url: Path<String>) -> impl Responder {
    posts.find(Uuid::parse_str(&url.into_inner()).unwrap()).first::<Post>(&mut conn(data)).map_or_else(
        |_| HttpResponse::NoContent().body("No posts found"), 
        |post| HttpResponse::Ok().json(post)
    )
}

#[post("/posts")]
async fn create_post(data: Data<DbPool>, form: Json<_CreatePost>, req: HttpRequest) -> impl Responder {
    if let Some(cookie) = req.cookie("token") {
        match decode_token(cookie.value()) {
            Ok(id) => {
                let new_post = Post {
                    post_id: Uuid::new_v4(),
                    author_id: id,
                    thumbnail: Some(String::from("tn")),
                    content: form.content.clone(),
                    sparks: 0,
                    echoes: 0,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now()
                };

                diesel::insert_into(posts).values(new_post).execute(&mut conn(data)).map_or_else(
                    |_| HttpResponse::InternalServerError().finish(),
                    |_| HttpResponse::Ok().finish()
                )
            },
            Err(_) => HttpResponse::Unauthorized().finish()
        }
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

#[patch("/posts/{post_id}")]
async fn update_post(data: Data<DbPool>, form: Json<UpdatePost>, url: Path<String>) -> impl Responder {
    let id = Uuid::parse_str(&url.into_inner()).unwrap();

    let updated = UpdatePost {
        content: form.content.clone(),
        updated_at: chrono::Utc::now()
    };

    diesel::update(posts.find(id)).set(updated).execute(&mut conn(data)).map_or_else(
        |_| HttpResponse::InternalServerError().finish(),
        |_| HttpResponse::Ok().finish()
    )
}

#[delete("/posts/{post_id}")]
async fn delete_post(data: Data<DbPool>, req: HttpRequest) -> impl Responder {
    if let Some(cookie) = req.cookie("token") {
        match decode_token(cookie.value()) {
            Ok(id) => {
                diesel::delete(posts.find(id)).execute(&mut conn(data)).map_or_else(
                    |_| HttpResponse::InternalServerError().finish(),
                    |_| HttpResponse::Ok().finish()
                )                
            },
            Err(_) => HttpResponse::Unauthorized().finish()
        }
    } else {
        HttpResponse::Unauthorized().body("You're not logged in")
    }
}
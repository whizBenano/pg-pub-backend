use actix_web::{HttpRequest, HttpResponse, Responder, get, post, web::{Data, Json, Path}};
use super::{super::config::DbPool, super::schema::books::dsl::*, book_model::{Book, _CreateBook}, super::user::user_services::{conn, decode_token}};
use diesel::prelude::*;
use uuid::Uuid;

#[get("/books")]
async fn get_books(data: Data<DbPool>) -> impl Responder {
    books.load::<Book>(&mut conn(data)).map_or_else(
        |_| HttpResponse::NoContent().body("No books yet"),
        |list| HttpResponse::Ok().json(list)
    )
}

#[get("/books/{id}")]
async fn get_book(data: Data<DbPool>, url: Path<String>) -> impl Responder {
    books.find(Uuid::parse_str(&url.into_inner()).unwrap()).first::<Book>(&mut conn(data)).map_or_else(
        |_| HttpResponse::NoContent().body("No books yet"),
        |book| HttpResponse::Ok().json(book)
    )
}

#[post("/books")]
async fn create_book(data: Data<DbPool>, form: Json<_CreateBook>, req: HttpRequest) -> impl Responder {
    if let Some(cookie) = req.cookie("token") {
        match decode_token(cookie.value()) {
            Ok(id) => {
                let new_book = Book {
                    author_id: id,
                    book_id: Uuid::new_v4(),
                    book_title: form.book_title.clone(),
                    content: Some(form.content.clone()),
                    price: form.price,
                    rating: 0.0,
                    created_at: chrono::Utc::now(),
                    synopsis: form.synopsis.clone(),
                    img_url: Some(form.img_url.clone())
                };

                diesel::insert_into(books).values(new_book).execute(&mut conn(data)).map_or_else(
                    |_| HttpResponse::InternalServerError().body("Failed to create book"),
                    |_| HttpResponse::Ok().body("Book created successfully")
                )
            },
            Err(ex) => HttpResponse::Unauthorized().body(format!("Invalid token: {}", ex))
        }
    } else {
        HttpResponse::Unauthorized().body("You're not logged in")
    }
}

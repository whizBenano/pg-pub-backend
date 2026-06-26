use super::{
    user::user_handlers::{ create_user, login_user, dashboard, update_user, delete_user, logout_user, profile, index },
    book::{ transformer::convert, book_handlers::{ get_books, get_book } }, 
    post::post_handlers::{ create_post, get_posts, get_post, update_post, delete_post }
};



pub fn app_config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
    .service(convert)
    .service(index)
    
    // Posts
    .service(create_post)
    .service(get_posts)
    .service(get_post)
    .service(update_post)
    .service(delete_post)

    // Books
    .service(get_books)
    .service(get_book)

    // Users
    .service(create_user)
    .service(login_user)
    .service(dashboard)
    .service(update_user)
    .service(delete_user)
    .service(logout_user)
    .service(profile);
}
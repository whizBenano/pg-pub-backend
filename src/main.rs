mod schema;
mod program;
use actix_web::{App, HttpServer, middleware::{Logger, NormalizePath}, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();
    let data = web::Data::new(program::model::init_pool().await);
    let (host, port) =
        (
            std::env::var("HOST").map_err(|_| "Host not available").unwrap(),
            std::env::var("PORT").map_err(|_| "Port not available").unwrap()
        );

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .configure(program::endpoints::app_config)
            .wrap(program::model::cors())
            .wrap(Logger::default())
            .wrap(NormalizePath::trim())
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
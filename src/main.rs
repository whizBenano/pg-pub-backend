mod schema;
mod program;
use actix_web::{App, HttpServer, middleware::{Logger, NormalizePath}, web};

use rustls::{pki_types::{CertificateDer, PrivateKeyDer}, ServerConfig,};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::{fs::File, io::{BufReader}};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let config = load_rustls_config();
    let data = web::Data::new(program::model::init_pool().await);
    let (host, port) = (
        std::env::var("HOST").map_err(|_| "Host not found").unwrap(),
        std::env::var("PORT").map_err(|_| "Port not available").unwrap()
    );
    
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .configure(program::endpoints::app_config)
            .wrap(program::model::cors())
            .wrap(NormalizePath::trim())
            .wrap(Logger::default())
    })
    .bind_rustls_0_23(format!("{}:{}", host, port), config)?
    .run()
    .await
}

fn load_rustls_config() -> ServerConfig {

    // Load certificate
    let cert_file = &mut BufReader::new(
        File::open("cert.pem").unwrap()
    );

    let cert_chain: Vec<CertificateDer<'static>> =
        certs(cert_file)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

    // Load private key
    let key_file = &mut BufReader::new(
        File::open("key.pem").unwrap()
    );

    let mut keys: Vec<PrivateKeyDer<'static>> =
        pkcs8_private_keys(key_file)
            .map(|key| key.map(Into::into))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

    let key = keys.remove(0);

    ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key)
        .unwrap()
}
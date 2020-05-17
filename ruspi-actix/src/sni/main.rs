use std::fs::File;
use std::io::BufReader;

use actix_files::Files;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, middleware, web};
use rustls::{NoClientAuth, ServerConfig, ResolvesServerCertUsingSNI};
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::sign::{RSASigningKey, SigningKey};
use std::sync::Arc;

fn add_certificate_to_resolver(
    name: &str, hostname: &str,
    resolver: &mut ResolvesServerCertUsingSNI,
) {
    let cert_file = &mut BufReader::new(File::open(
        format!("certificates/rsa/{}.ruspi.dev-fullchain.crt", name)
    ).unwrap());
    let key_file = &mut BufReader::new(File::open(
        format!("certificates/rsa/{}.ruspi.dev.key", name)
    ).unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = pkcs8_private_keys(key_file).unwrap();
    let signing_key = RSASigningKey::new(
        &keys.remove(0)
    ).unwrap();
    let signing_key_boxed: Arc<Box<dyn SigningKey>> = Arc::new(
        Box::new(signing_key)
    );

    resolver.add(hostname, rustls::sign::CertifiedKey::new(
        cert_chain, signing_key_boxed,
    )).expect(&format!("Invalid certificate {}", name));
}

/// simple handle
async fn index(req: HttpRequest) -> HttpResponse {
    println!("{:?}", req);
    HttpResponse::Ok()
        .content_type("text/plain")
        .body("Welcome!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    // load ssl keys
    let mut resolver = ResolvesServerCertUsingSNI::new();

    add_certificate_to_resolver("test1", "test1.ruspi.dev", &mut resolver);
    add_certificate_to_resolver("test2", "test2.ruspi.dev", &mut resolver);
    add_certificate_to_resolver("test3", "test3.ruspi.dev", &mut resolver);


    let mut config = ServerConfig::new(NoClientAuth::new());

    config.cert_resolver = Arc::new(resolver);

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            // register simple handler, handle all methods
            .service(web::resource("/index.html").to(index))
            // with path parameters
            .service(web::resource("/").route(web::get().to(|| {
                HttpResponse::Found()
                    .header("LOCATION", "/index.html")
                    .finish()
            })))
            .service(Files::new("/static", "static"))
    })
        .bind_rustls("127.0.0.1:8443", config)?
        .run()
        .await
}
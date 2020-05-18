use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, middleware};
use dotenv::dotenv;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hi again!")
}

#[get("/hello")]
async fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    env_logger::init();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
            .route("/hi", web::get().to(index2))
            .service(index3)
    })
        .bind("127.0.0.1:9080")?
        .bind("127.0.0.1:9081")?
        .bind("127.0.0.1:9082")?
        .run()
        .await
}
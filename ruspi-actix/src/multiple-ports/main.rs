use std::sync::Mutex;

use actix_web::{App, get, HttpResponse, HttpServer, middleware, Responder, web};
use actix_web::web::{Data, Path};

use dotenv::dotenv;
use crate::sse::Broadcaster;

mod sse;

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

async fn sse() -> impl Responder {
    let content = include_str!("sse.html");

    HttpResponse::Ok()
        .header("content-type", "text/html")
        .body(content)
}

async fn new_client(broadcaster: Data<Mutex<Broadcaster>>) -> impl Responder {
    let rx = broadcaster.lock().unwrap().new_client();

    HttpResponse::Ok()
        .header("content-type", "text/event-stream")
        .no_chunking()
        .streaming(rx)
}

async fn broadcast(
    msg: Path<String>,
    broadcaster: Data<Mutex<Broadcaster>>,
) -> impl Responder {
    broadcaster.lock().unwrap().send(&msg.into_inner());

    HttpResponse::Ok().body("msg sent")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    let data = Broadcaster::create();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(data.clone())
            .route("/", web::get().to(index))
            .route("/hi", web::get().to(index2))
            .route("/sse", web::get().to(sse))
            .route("/events", web::get().to(new_client))
            .route("/broadcast/{msg}", web::get().to(broadcast))
            .service(index3)
    })
        .bind("127.0.0.1:9080")?
        .bind("127.0.0.1:9081")?
        .bind("127.0.0.1:9082")?
        .run()
        .await
}
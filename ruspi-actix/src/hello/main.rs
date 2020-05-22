use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, error, HttpRequest, FromRequest};
use serde::{Deserialize, Serialize};

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

#[derive(Deserialize)]
pub struct SayHello {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct Hi {
    pub value: String,
}

fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    use actix_web::error::JsonPayloadError;

    let detail = err.to_string();
    let resp = match &err {
        JsonPayloadError::ContentType => {
            HttpResponse::UnsupportedMediaType().body(detail)
        }
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity().body(detail)
        }
        _ => HttpResponse::BadRequest().body(detail),
    };
    error::InternalError::from_response(err, resp).into()
}

#[post("/hip")]
async fn hi(sh: web::Json<SayHello>) -> impl Responder {

    let res = Hi {
        value: format!("Hello: {}", sh.name.clone())
    };

    HttpResponse::Ok().json(res)
}



#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/hi", web::get().to(index2))
            .service(index3)
            .service(hi)
            .app_data(web::Json::<SayHello>::configure(|cfg| {
                cfg.error_handler(json_error_handler)
            }))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
use dotenv::dotenv;

mod config;
mod server;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    env_logger::init();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "warn,tlsproxy=info,actix_web=info");
    }



    server::start().await
}
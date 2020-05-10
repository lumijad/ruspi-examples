/*

The following command creates your self signed certificate with openssl:
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes


Ignore self signed certificate
curl -k https://127.0.0.1:8443/warp

*/


use std::env;

use warp::Filter;

fn get_routes() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::any()
        .and(warp::get())
        .and(warp::path::param())
        .and(warp::header("user-agent"))
        .map(|param: String, agent: String| {
            format!("Hello {}, whose agent is {}", param, agent)
        })
}

#[tokio::main]
async fn main() {

    // https://github.com/seanmonstar/pretty-env-logger
    env::set_var("RUST_LOG", "info,tls=debug");

    pretty_env_logger::init();

    println!("Starting warp with the bind address 127.0.0.1:8443");


    warp::serve(get_routes().with(warp::log("warp-server")))
        .tls().cert_path("tls/cert.pem")
        .key_path("tls/key.pem").run(([127, 0, 0, 1], 8443)).await;
}

#[cfg(test)]
mod tests {
    use warp::test::request;

    use crate::get_routes;

    #[tokio::test]
    async fn test_server() {
        let resp = request()
            .method("GET")
            .header("user-agent", "test")
            .path("https://127.0.0.1:8443/warp")
            .reply(&get_routes())
            .await;

        let body = std::str::from_utf8(resp.body()).unwrap().to_string();
        println!("{}", body);

        assert_eq!(resp.status(), 200);
    }
}
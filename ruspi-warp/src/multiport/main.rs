use futures::{prelude::*};
use warp::Filter;
use futures::channel::mpsc;

/*

curl -k https://127.0.0.1:8443/hi/warp
curl -k https://127.0.0.1:8443/shutdown

 */

fn get_routes() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path("hi")
        .and(warp::get())
        .and(warp::path::param())
        .and(warp::header("user-agent"))
        .map(|param: String, agent: String| {
            format!("Hello {}, whose agent is {}", param, agent)
        })
}

#[tokio::main]
async fn main() {

    let (stop_http_tx, stop_http_rx) = mpsc::channel::<()>(0);
    let stop_http_rx = stop_http_rx.into_future().map(|_| ());

    let (stop_https_tx, stop_https_rx) = mpsc::channel::<()>(0);
    let stop_https_rx = stop_https_rx.into_future().map(|_| ());

    let routes = get_routes();

    let routes = routes.or(warp::path("shutdown").and(warp::get())
        .map(move || {
            stop_http_tx.clone().try_send(()).unwrap();
            stop_https_tx.clone().try_send(()).unwrap();
            format!("Stopping server")
        }));


    let (_addr, warp_http) = warp::serve(routes.clone().with(warp::log("warp-server")))
        .bind_with_graceful_shutdown(([127, 0, 0, 1], 8080), stop_http_rx);

    let (_addr, warp_https) = warp::serve(routes.with(warp::log("warp-server")))
        .tls().cert_path("tls/cert.pem")
        .key_path("tls/key.pem").bind_with_graceful_shutdown(([127, 0, 0, 1], 8443), stop_https_rx);

    future::join(warp_http, warp_https).await;

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
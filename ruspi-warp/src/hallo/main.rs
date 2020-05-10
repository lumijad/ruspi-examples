use warp::Filter;

/*

[dependencies]
tokio = { version = "0.2", features = ["macros"] }
warp = "0.2"

*/

#[tokio::main]
async fn main() {
    // All requests get "Hello World!" back
    let routes = warp::any().map(|| "Hello, World!!");

    println!("Starting warp with the bind address 127.0.0.1:8080");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

/*

curl examples

Only Payload
curl localhost:8080

With Header
curl -I -localhost:8080

Windows 10 supports curl via powershell:
powershell curl -Uri http://localhost:8080

 */

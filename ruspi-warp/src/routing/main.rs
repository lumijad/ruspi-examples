extern crate log;
extern crate pretty_env_logger;

use std::env;

use warp::Filter;

fn get_routes() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {

    // Note that composing filters for many routes may increase compile times (because it uses a lot of generics).
    // If you wish to use dynamic dispatch instead and speed up compile times while
    // making it slightly slower at runtime, you can use Filter::boxed().

    // We'll start simple, and gradually show how you combine these powers
    // into super powers!

    // GET /hi
    let hi = warp::path("hi").map(|| "Hello, World!");

    // How about multiple segments? First, we could use the `path!` macro:
    //
    // GET /hello/from/warp
    let hello_from_warp = warp::path!("hello" / "from" / "warp").map(|| "Hello from warp!");

    // Fine, but how do I handle parameters in paths?
    //
    // GET /sum/:u32/:u32
    let sum = warp::path!("sum" / u32 / u32).map(|a, b| format!("{} + {} = {}", a, b, a + b));

    // Any type that implements FromStr can be used, and in any order:
    //
    // GET /:u16/times/:u16
    let times =
        warp::path!(u16 / "times" / u16).map(|a, b| format!("{} times {} = {}", a, b, a * b));

    // Oh shoot, those math routes should be mounted at a different path,
    // is that possible? Yep.
    //
    // GET /math/sum/:u32/:u32
    // GET /math/:u16/times/:u16
    let math = warp::path("math");
    let _sum = math.and(sum);
    let _times = math.and(times);

    // What! And? What's that do?
    //
    // It combines the filters in a sort of "this and then that" order. In
    // fact, it's exactly what the `path!` macro has been doing internally.
    //
    // GET /bye/:string
    let bye = warp::path("bye")
        .and(warp::path::param())
        .map(|name: String| format!("Good bye, {}!", name));

    // Ah, can filters do things besides `and`?
    //
    // Why, yes they can! They can also `or`! As you might expect, `or` creates
    // a "this or else that" chain of filters. If the first doesn't succeed,
    // then it tries the other.
    //
    // So, those `math` routes could have been mounted all as one, with `or`.
    //
    // GET /math/sum/:u32/:u32
    // GET /math/:u16/times/:u16
    let math = warp::path("math").and(sum.or(times));

    // We can use the end() filter to match a shorter path
    let help = warp::path("math")
        // Careful! Omitting the following line would make this filter match
        // requests to /math/sum/:u32/:u32 and /math/:u16/times/:u16
        .and(warp::path::end())
        .map(|| "This is the Math API. Try calling /math/sum/:u32/:u32 or /math/:u16/times/:u16");
    let math = help.or(math);

    // Let's let people know that the `sum` and `times` routes are under `math`.
    let sum = sum.map(|output| format!("(This route has moved to /math/sum/:u16/:u16) {}", output));
    let times =
        times.map(|output| format!("(This route has moved to /math/:u16/times/:u16) {}", output));

    // It turns out, using `or` is how you combine everything together into
    // a single API. (We also actually haven't been enforcing the that the
    // method is GET, so we'll do that too!)
    //
    // GET /hi
    // GET /hello/from/warp
    // GET /bye/:string
    // GET /math/sum/:u32/:u32
    // GET /math/:u16/times/:u16

    let routes = warp::get().and(hi.or(hello_from_warp).or(bye).or(math).or(sum).or(times));

    routes
}

#[tokio::main]
async fn main() {
    // https://github.com/seanmonstar/pretty-env-logger
    env::set_var("RUST_LOG", "info,routing=debug");

    pretty_env_logger::init();

    println!("Starting warp with the bind address 127.0.0.1:8080");
    warp::serve(get_routes().with(warp::log("warp-server"))).run(([127, 0, 0, 1], 8080)).await;
}


#[cfg(test)]
mod tests {
    use warp::test::request;

    use crate::get_routes;

    async fn execute_get(url: &String) {
        println!("URL: {}", url);

        let resp = request()
            .method("GET")
            .path(url)
            .reply(&get_routes())
            .await;

        let body = std::str::from_utf8(resp.body()).unwrap().to_string();
        println!("{}", body);
    }

    #[tokio::test]
    async fn test_server() {
        let gets = vec!["/hi", "/hello/from/warp", "/bye/warp", "/math/sum/3/5", "/math/4/times/5"];

        for get in gets {
            let url = "http://localhost:8080".to_owned() + get;
            execute_get(&url).await;
        }
    }
}
use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();


    // dir already requires GET...
    let css = warp::path("css").and(warp::fs::dir("./static/css"));
    let font = warp::path("font").and(warp::fs::dir("./static/font"));
    let js = warp::path("js").and(warp::fs::dir("./static/js"));

    let routes = css.or(font).or(js);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
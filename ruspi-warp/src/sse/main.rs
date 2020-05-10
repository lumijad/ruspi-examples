use futures::StreamExt;
use std::convert::Infallible;
use std::time::Duration;
use tokio::time::interval;
use warp::{sse::ServerSentEvent, Filter};

// create server-sent event
fn sse_counter(counter: u64) -> Result<impl ServerSentEvent, Infallible> {
    Ok(warp::sse::data(counter))
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let routes = warp::path("ticks").and(warp::get()).map(|| {
        let mut counter: u64 = 0;
        // create server event source
        let event_stream = interval(Duration::from_secs(1)).map(move |_| {
            counter += 1;
            sse_counter(counter)
        });
        // reply using server-sent events
        warp::sse::reply(event_stream)
    }).or(warp::path::end().map(|| {
        warp::http::Response::builder()
            .header("content-type", "text/html; charset=utf-8")
            .body(INDEX_HTML)
    }));


    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

static INDEX_HTML: &str = r#"
<!DOCTYPE html>
<html>
    <head>
        <title>Server side event</title>
    </head>
    <body>
        <h1>Server side event</h1>
        <div id="sse_ctl">
            <p>Connecting...></p>
        </div>

        <script type="text/javascript">
        var uri = 'http://localhost:8080/ticks';

        var sse = new EventSource(uri);
        function message(data) {
            var line = document.createElement('p');
            line.innerText = data;
            sse_ctl.removeChild(sse_ctl.childNodes[0]);
            sse_ctl.appendChild(line);
        }

        sse.onopen = function() {
            sse_ctl.innerHTML = "<p><em>Connected!</em></p>";
        }

        sse.onmessage = function(msg) {
            message(msg.data);
        };
        </script>
    </body>
</html>
"#;
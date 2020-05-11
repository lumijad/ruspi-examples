use futures::{FutureExt, StreamExt};
use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let echo = warp::path("echo")
        // The `ws()` filter will prepare the Websocket handshake.
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            // And then our closure will be called when it completes...
            ws.on_upgrade(|websocket| {
                // Just echo all messages back...
                let (tx, rx) = websocket.split();
                rx.forward(tx).map(|result| {
                    if let Err(e) = result {
                        eprintln!("websocket error: {:?}", e);
                    }
                })
            })
        });

    // GET / -> index html
    let index = warp::path::end().map(|| warp::reply::html(INDEX_HTML));

    let routes = index.or(echo);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

static INDEX_HTML: &str = r#"
<!DOCTYPE html>
<html>
    <head>
        <title>Warp Echo Server</title>
    </head>
    <body>
        <h1>warp echo server</h1>
        <div id="echo">
            <p><em>Connecting...</em></p>
        </div>
        <input type="text" id="text" />
        <button type="button" id="send">Send</button>
        <script type="text/javascript">
        var uri = 'ws://' + location.host + '/echo';
        var ws = new WebSocket(uri);

        function message(data) {
            var line = document.createElement('p');
            line.innerText = data;
            echo.appendChild(line);
        }

        ws.onopen = function() {
            echo.innerHTML = "<p><em>Connected!</em></p>";
        }

        ws.onmessage = function(msg) {
            message(msg.data);
        };

        send.onclick = function() {
            var msg = text.value;
            ws.send('Echo:' + msg);
            text.value = '';

            message('You:' + msg);
        };
        </script>
    </body>
</html>
"#;

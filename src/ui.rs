

use warp::Filter;
use warp::ws::{WebSocket, Message};
use futures_util::{stream::StreamExt, sink::SinkExt};
use tokio::sync::mpsc;
use tokio::task;

pub async fn start_server() {
    let routes = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(handle_ws_connection)
        });

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn handle_ws_connection(websocket: WebSocket) {
    let (mut tx, mut rx) = websocket.split();
    let (data_tx, mut data_rx) = mpsc::unbounded_channel::<String>();

    task::spawn(async move {
        while let Some(data) = data_rx.recv().await {
            if tx.send(Message::text(data)).await.is_err() {
                break;
            }
        }
    });

    while let Some(result) = rx.next().await {
        if let Ok(msg) = result {
            if let Ok(text) = msg.to_str() {
                match text {
                    "start" => {
                        let _ = data_tx.send("Starting packet capture...".to_string());
                    },
                    "stop" => {
                        let _ = data_tx.send("Stopping packet capture...".to_string());
                    },
                    _ => {
                        let _ = data_tx.send(format!("Received unknown command: {}", text));
                    }
                }
            }
        }
    }
}

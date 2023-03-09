use std::net::SocketAddr;
use axum::{
    routing::get,
    Router,
    extract::ws::{WebSocketUpgrade, WebSocket},
    response::{Response},
};


#[tokio::main]
async fn main() {
  let app = Router::new()
      .route("/", get(|| async {"Hello World"}))
      .route("/ws", get(handler));

  let addr = SocketAddr::from(([0,0,0,0], 3000));
  axum::Server::bind(&addr)
      .serve(app.into_make_service())
      .await
      .unwrap();
}

async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            return;
        };

        if socket.send(msg).await.is_err() {
            return;
        }
    }
}


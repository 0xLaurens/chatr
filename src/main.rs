use std::net::SocketAddr;
use std::sync::Arc;
use axum::{
    routing::get,
    Router,
    extract::ws::{WebSocketUpgrade, WebSocket},
    response::{Response},
};
use axum::extract::State;
use axum::extract::ws::Message;
use axum::middleware::from_fn_with_state;
use axum::response::{Html, IntoResponse};
use futures::{SinkExt, StreamExt};
use tokio::sync::broadcast;
use tokio::select;

struct AppState {
    tx: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() {
    let (tx, _rx) = broadcast::channel(69);
    let app_state = Arc::new(AppState { tx });
    let app = Router::new()
        .route("/", get(index))
        .route("/ws", get(handler))
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(ws: WebSocketUpgrade,
                 State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();

    let tx = state.tx.clone();
    let mut rx = tx.subscribe();

    let _ = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let _ = {
        let tx = tx.clone();

        tokio::spawn(async move {
            while let Some(Ok(Message::Text(text))) = receiver.next().await {
                let _ = tx.send(format!("{}", text));
            }
        })
    };
}

async fn index() -> Html<&'static str> {
    Html(std::include_str!("../chat.html"))
}
use std::collections::HashSet;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use axum::{
    routing::get,
    Router,
    extract::ws::{WebSocketUpgrade, WebSocket, Message},
};
use axum::extract::State;
use axum::response::{IntoResponse};
use futures::{SinkExt, StreamExt};
use tokio::sync::{broadcast};

struct AppState {
    users: Mutex<HashSet<String>>,
    tx: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT")
        .map(|val| val.parse::<u16>())
        .unwrap_or(Ok(3000)).unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let (tx, _rx) = broadcast::channel(69);
    let users = Mutex::new(HashSet::new());
    let app_state = Arc::new(AppState { users, tx });
    let app = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/ws", get(handler))
        .with_state(app_state);

    println!("Hosted on {}", addr.to_string());
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

    let mut username = String::new();
    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(user) = msg {
            check_username(&user, &state, &mut username);
            if !username.is_empty() {
                break;
            } else {
                let _ = sender
                    .send(Message::Text(String::from("Username already taken.")))
                    .await;
            }
        }
    }


    let tx = state.tx.clone();
    let mut rx = tx.subscribe();

    let joined = format!("{} joined the chat!", username);
    let _ = tx.send(joined);

    let mut recv_messages = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let mut send_messages = {
        let tx = tx.clone();
        let name = username.clone();

        tokio::spawn(async move {
            while let Some(Ok(Message::Text(text))) = receiver.next().await {
                let _ = tx.send(format!("{}: {}", name, text));
            }
        })
    };

    tokio::select! {
        _ = (&mut send_messages) => recv_messages.abort(),
        _ = (&mut recv_messages) => send_messages.abort(),
    }
    ;

    let left = format!("{} left the chat!", username);
    let _ = tx.send(left);
}

fn check_username(user: &str, state: &AppState, username: &mut String) {
    let mut users = state.users.lock().unwrap();
    if !users.contains(user) {
        users.insert(user.to_owned());
        username.push_str(user);
    }
}
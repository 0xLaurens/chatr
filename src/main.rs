use std::collections::{HashMap, HashSet};
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
use serde::Deserialize;
use serde_json::json;

struct AppState {
    rooms: Mutex<HashMap<String, RoomState>>,
}

struct RoomState {
    users: Mutex<HashSet<String>>,
    tx: broadcast::Sender<String>,
}

impl RoomState {
    fn new() -> Self {
        Self {
            users: Mutex::new(HashSet::new()),
            tx: broadcast::channel(69).0,
        }
    }
}

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT")
        .map(|val| val.parse::<u16>())
        .unwrap_or(Ok(3000)).unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let app_state = Arc::new(AppState {
        rooms: Mutex::new(HashMap::new())
    });

    let app = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/ws", get(handler))
        .route("/rooms", get(get_rooms))
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
    let mut channel = String::new();
    let mut tx = None::<broadcast::Sender<String>>;

    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(name) = msg {
            #[derive(Deserialize)]
            struct Connect {
                username: String,
                channel: String,
            }

            let connect: Connect = match serde_json::from_str(&name) {
                Ok(connect) => connect,
                Err(err) => {
                    println!("{}", err);
                    let _ = sender.send(Message::from("Failed to connect to room!")).await;
                    break;
                }
            };

            {
                let mut rooms = state.rooms.lock().unwrap();
                channel = connect.channel.clone();

                let room = rooms.entry(connect.channel).or_insert_with(RoomState::new);
                tx = Some(room.tx.clone());

                if !room.users.lock().unwrap().contains(&connect.username) {
                    room.users.lock().unwrap().insert(connect.username.to_owned());
                    username = connect.username.clone();
                }
            }

            if tx.is_some() && !username.is_empty() {
                break;
            } else {
                let _ = sender
                    .send(Message::Text(String::from("Username already taken.")))
                    .await;

                return;
            }
        }
    };

    let tx = tx.unwrap();
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
    let mut rooms = state.rooms.lock().unwrap();
    rooms.get_mut(&channel).unwrap().users.lock().unwrap().remove(&username);

    if rooms.get_mut(&channel).unwrap().users.lock().unwrap().len() == 0 {
        rooms.remove(&channel);
    }
}

async fn get_rooms(State(state): State<Arc<AppState>>) -> String {
    let rooms = state.rooms.lock().unwrap();
    let vec = rooms.keys().into_iter().collect::<Vec<&String>>();
    match vec.len() {
        0 => json!({
            "status": "No rooms found yet!",
            "rooms": []
        }).to_string(),
        _ => json!({
            "status": "Success!",
            "rooms": vec
        }).to_string()
    }
}
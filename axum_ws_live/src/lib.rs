mod msg;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        Extension, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use dashmap::{DashMap, DashSet};
use futures::{SinkExt, StreamExt};
pub use msg::{Msg, MsgData};
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::log::warn;

const CAPACITY: usize = 64;

#[derive(Debug)]
struct State {
    // for a given user, how many rooms they're in
    user_rooms: DashMap<String, DashSet<String>>,
    // for a given room, how many users are in it
    room_users: DashMap<String, DashSet<String>>,
    tx: broadcast::Sender<Arc<Msg>>,
}

#[derive(Debug, Clone, Default)]
pub struct ChatState(Arc<State>);

impl Default for State {
    fn default() -> Self {
        let (tx, _rx) = broadcast::channel(CAPACITY);
        Self {
            user_rooms: Default::default(),
            room_users: Default::default(),
            tx,
        }
    }
}

impl ChatState {
    pub fn new() -> Self {
        Self(Arc::new(State::default()))
    }

    pub fn get_user_rooms(&self, username: &str) -> Vec<String> {
        self.0
            .user_rooms
            .get(username)
            .map(|rooms| rooms.clone().into_iter().collect())
            .unwrap_or_default()
    }

    pub fn get_room_users(&self, room: &str) -> Vec<String> {
        self.0
            .room_users
            .get(room)
            .map(|users| users.clone().into_iter().collect())
            .unwrap_or_default()
    }
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(state): Extension<ChatState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: ChatState) {
    let mut rx = state.0.tx.subscribe();
    let (mut sender, mut receiver) = socket.split();

    let state1 = state.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(data)) = receiver.next().await {
            match data {
                Message::Text(msg) => {
                    handle_message(msg.as_str().try_into().unwrap(), state1.0.clone()).await;
                }
                _ => (),
            }
        }
    });

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let data = msg.as_ref().try_into().unwrap();
            if sender.send(Message::Text(data)).await.is_err() {
                warn!("failed to send message");
                break;
            }
        }
    });

    // if any of the tasks fail, we need to shut down the other one
    tokio::select! {
        _v1 = &mut recv_task => send_task.abort(),
        _v1 = &mut send_task => recv_task.abort(),
    }

    // this user has left. Should send a leave message to all romms
    // usually we can get username from auth header, here wo just use "fake_user"
    let username = "fake_user";
    warn!("connection for {username} closed");

    for room in state.get_room_users(username) {
        if let Err(e) = state.0.tx.send(Arc::new(Msg::leave(&room, username))) {
            warn!("failed to send leave message: {e}");
        };
    }
}

async fn handle_message(msg: Msg, state: Arc<State>) {
    let msg = match msg.data {
        MsgData::Join => {
            let username = msg.username.clone();
            let room = msg.room.clone();
            state
                .user_rooms
                .entry(username.clone())
                .or_insert_with(DashSet::new)
                .insert(room.clone());
            state
                .room_users
                .entry(room)
                .or_insert_with(DashSet::new)
                .insert(username);
            msg
        }
        MsgData::Leave => {
            if let Some(v) = state.user_rooms.get_mut(&msg.username) {
                v.remove(&msg.room);
                if v.is_empty() {
                    state.user_rooms.remove(&msg.username);
                }
            }

            if let Some(v) = state.room_users.get_mut(&msg.room) {
                v.remove(&msg.username);
                if v.is_empty() {
                    state.user_rooms.remove(&msg.room);
                }
            }

            msg
        }
        _ => msg,
    };
    if let Err(e) = state.tx.send(Arc::new(msg)) {
        warn!("error sending message: {e}");
    }
}

use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Msg {
    pub room: String,
    pub username: String,
    pub timestamp: u64,
    pub data: MsgData,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MsgData {
    Join,
    Leave,
    Message(String),
}

impl TryFrom<&str> for Msg {
    type Error = serde_json::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(s)
    }
}

impl TryFrom<&Msg> for String {
    type Error = serde_json::Error;

    fn try_from(msg: &Msg) -> Result<Self, Self::Error> {
        serde_json::to_string(msg)
    }
}

impl Msg {
    pub fn new(room: String, username: String, data: MsgData) -> Self {
        Msg {
            room,
            username,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            data,
        }
    }

    pub fn join(room: &str, username: &str) -> Self {
        Msg::new(room.into(), username.into(), MsgData::Join)
    }

    pub fn leave(room: &str, username: &str) -> Self {
        Msg::new(room.into(), username.into(), MsgData::Leave)
    }

    pub fn message(room: &str, username: &str, message: &str) -> Self {
        Msg::new(
            room.into(),
            username.into(),
            MsgData::Message(message.into()),
        )
    }
}

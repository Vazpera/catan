use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    // Client Messages
    JoinNetwork(String),
    LeaveNetwork(String),
    Incriment,
    // Server Messages
    Kick(String),
    // Bi-directional
    SendMessage(String),
    Ping,
}

impl Message {
    pub fn parse(data: &[u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize(data).map_err(|e| e.into())
    }

    pub fn serialize(&self) -> Result<Vec<u8>, bincode::Error> {
        bincode::serialize(self).map_err(|e| e.into())
    }
    pub fn eval(&self, app: Arc<crate::App>) {
        match self {
            Self::Incriment => app.increment_counter(),
            _ => {},
        }
    }
}

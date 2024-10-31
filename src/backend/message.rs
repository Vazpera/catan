use std::{cmp::Ordering, sync::Arc};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    GuessNumber(u8),
    Result(bool),
}

impl Message {
    pub fn parse(data: &[u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize(data).map_err(|e| e.into())
    }

    pub fn serialize(&self) -> Result<Vec<u8>, bincode::Error> {
        bincode::serialize(self).map_err(|e| e.into())
    }
    pub async fn eval(&self, app: Arc<crate::App>) {
        match (self, *app.is_host.lock().unwrap()) {
            (Self::GuessNumber(j), true) => app.test_guess(j.to_owned()).await,
            (Self::Result(j), false) => app.process_result(j.to_owned()).await,
            _ => {}
        }
    }
}

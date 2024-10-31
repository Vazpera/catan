use std::{error, sync::Mutex};

use futures::{
    channel::mpsc::{self, Receiver, Sender},
    SinkExt,
};

use crate::backend::message::Message;
/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
pub struct App {
    pub running: Mutex<bool>,
    pub counter: Mutex<u8>,
    pub is_host: Mutex<bool>,
    pub guessed_correct: Mutex<Option<bool>>,
    pub connections: Mutex<u8>,
    pub message_stream: (Sender<Message>, Mutex<Receiver<Message>>),
}

impl Default for App {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel(1024);
        Self {
            running: Mutex::new(true),
            counter: Mutex::new(0),
            is_host: Mutex::new(false),
            guessed_correct: Mutex::new(None),
            connections: Mutex::new(0),
            message_stream: (tx, Mutex::new(rx)),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&self) {
        *self.running.lock().unwrap() = false;
    }

    pub fn increment_counter(&self) {
        let mut cnter = self.counter.lock().unwrap();
        if let Some(res) = cnter.checked_add(1) {
            *cnter = res;
        }
    }

    pub fn decrement_counter(&self) {
        let mut cnter = self.counter.lock().unwrap();
        if let Some(res) = cnter.checked_sub(1) {
            *cnter = res;
        }
    }

    pub async fn test_guess(&self, guess: u8) {
        let correct = *self.counter.lock().unwrap() == guess;
        self.message_stream.0.clone().send(Message::Result(correct)).await.unwrap();
    }

    pub async fn publish_guess(&self) {
        self.message_stream
            .0
            .clone()
            .send(Message::GuessNumber(*self.counter.lock().unwrap()))
            .await.unwrap();
    }
    pub async fn process_result(&self, res: bool) {
        *self.guessed_correct.lock().unwrap() = Some(res);
    }
}

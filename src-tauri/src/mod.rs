use queues::Queue;
use rusqlite::Connection;
use std::sync::Mutex;

pub struct GlobalState {
    pub connection: std::sync::Mutex<rusqlite::Connection>,
    pub sink: Option<rodio::Sink>,
    pub queue: Queue<String>,
}

impl GlobalState {
    pub fn new(sink: rodio::Sink) -> Self {
        GlobalState {
            connection: Mutex::new(Connection::open_in_memory().unwrap()),
            sink: Some(sink),
            queue: Queue::new(),
        }
    }
}

impl Default for GlobalState {
    fn default() -> Self {
        GlobalState {
            connection: Mutex::new(Connection::open_in_memory().unwrap()),
            sink: Option::None,
            queue: Queue::new(),
        }
    }
}

pub mod album;
pub mod artist;
pub mod playlist;
pub mod track;
pub mod utilities;

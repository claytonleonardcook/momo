use rusqlite::Connection;
use std::sync::Mutex;

pub struct GlobalState {
    pub connection: std::sync::Mutex<rusqlite::Connection>,
    pub sink: Option<rodio::Sink>,
}

impl GlobalState {
    pub fn new(sink: rodio::Sink) -> Self {
        GlobalState {
            connection: Mutex::new(Connection::open_in_memory().unwrap()),
            sink: Some(sink),
        }
    }
}

impl Default for GlobalState {
    fn default() -> Self {
        GlobalState {
            connection: Mutex::new(Connection::open_in_memory().unwrap()),
            sink: Option::None,
        }
    }
}

pub mod album;
pub mod artist;
pub mod playlist;
pub mod track;
pub mod utilities;

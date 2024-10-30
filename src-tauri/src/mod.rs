pub struct GlobalState {
    pub connection: std::sync::Mutex<rusqlite::Connection>,
}

pub mod album;
pub mod artist;
pub mod playlist;
pub mod track;

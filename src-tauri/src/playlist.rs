use crate::{track::Track, GlobalState};
use include_sqlite_sql::{impl_sql, include_sql};
use serde::Serialize;
use std::sync::Mutex;
use tauri::State;

include_sql!("sql/Playlists.sql");
include_sql!("sql/PlaylistTracks.sql");

#[derive(Debug, Serialize)]
pub struct Playlist {
    pub id: i64,
    pub name: String,
}

impl Playlist {
    pub fn new(id: i64, name: String) -> Playlist {
        Playlist { id, name }
    }
}

impl std::clone::Clone for Playlist {
    fn clone(&self) -> Self {
        Playlist::new(self.id, self.name.clone())
    }
}

#[tauri::command]
pub fn get_all_playlists(global_state: State<Mutex<GlobalState>>) -> Result<Vec<Playlist>, String> {
    let state = global_state.lock().unwrap();

    let connnection = state.connection.lock().unwrap();

    let playlists = &mut Vec::new();

    connnection
        .get_all_playlists(|row| {
            let id: i64 = row.get_ref("id")?.as_i64()?;
            let name: String = row.get_ref("name")?.as_str()?.to_string();

            playlists.push(Playlist::new(id, name));

            Ok(())
        })
        .unwrap();

    Ok(playlists.to_vec())
}

#[tauri::command]
pub fn create_playlist(name: &str, global_state: State<Mutex<GlobalState>>) -> Result<i64, String> {
    let state = global_state.lock().unwrap();

    let connnection = state.connection.lock().unwrap();

    let playlist_id = connnection
        .insert_playlist(name, |row| Ok(row.get_ref("id")?.as_i64()?))
        .unwrap();

    Ok(playlist_id)
}

#[tauri::command]
pub fn add_track_to_playlist(
    playlist_id: i64,
    track_id: i64,
    global_state: State<Mutex<GlobalState>>,
) -> Result<(), String> {
    let state = global_state.lock().unwrap();

    let connnection = state.connection.lock().unwrap();

    connnection
        .add_track_to_playlist(playlist_id, track_id)
        .unwrap();

    Ok(())
}

#[tauri::command]
pub fn get_tracks_in_playlist(
    playlist_id: i64,
    global_state: State<Mutex<GlobalState>>,
) -> Result<Vec<Track>, String> {
    let state = global_state.lock().unwrap();

    let connnection = state.connection.lock().unwrap();

    let tracks = &mut Vec::new();

    connnection
        .get_tracks_in_playlist(playlist_id, |row| {
            let id: i64 = row.get_ref("id")?.as_i64()?;
            let name: String = row.get_ref("name")?.as_str()?.to_string();
            let path: String = row.get_ref("path")?.as_str()?.to_string();
            let album_id: i64 = row.get_ref("album_id")?.as_i64()?;

            tracks.push(Track::new(id, name, path, album_id));

            Ok(())
        })
        .unwrap();

    Ok(tracks.to_vec())
}

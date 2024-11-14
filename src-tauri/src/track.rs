use crate::GlobalState;
use audiotags::Tag;
use include_sqlite_sql::{impl_sql, include_sql};
use serde::Serialize;
use std::sync::Mutex;
use tauri::State;

include_sql!("sql/Tracks.sql");

#[derive(Debug, Serialize)]
pub struct Track {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub album_id: i64,
}

impl Track {
    pub fn new(id: i64, name: String, path: String, album_id: i64) -> Track {
        Track {
            id: id,
            name,
            path,
            album_id,
        }
    }
}

impl std::clone::Clone for Track {
    fn clone(&self) -> Self {
        Track::new(self.id, self.name.clone(), self.path.clone(), self.album_id)
    }
}

#[tauri::command]
pub fn get_all_tracks(global_state: State<Mutex<GlobalState>>) -> Result<Vec<Track>, String> {
    let state = global_state.lock().unwrap();

    let connnection = state.connection.lock().unwrap();

    let tracks = &mut Vec::new();

    connnection
        .get_all_tracks(|row| {
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

#[tauri::command]
pub fn get_tracks_by_artist(
    artist_name: &str,
    global_state: State<Mutex<GlobalState>>,
) -> Result<Vec<Track>, String> {
    let state = global_state.lock().unwrap();

    let connnection = state.connection.lock().unwrap();

    let tracks = &mut Vec::new();

    connnection
        .get_tracks_by_artist(artist_name, |row| {
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

#[tauri::command]
pub fn get_tracks_by_album(
    album_id: i64,
    global_state: State<Mutex<GlobalState>>,
) -> Result<Vec<Track>, String> {
    let state = global_state.lock().unwrap();

    let connnection = state.connection.lock().unwrap();

    let tracks = &mut Vec::new();

    connnection
        .get_tracks_by_album(album_id, |row| {
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

#[derive(serde::Serialize)]
pub struct TrackInformationResponse {
    pub track_title: String,
    pub album_title: String,
    pub artist_name: String,
    // pub track_duration: f64,
    pub track_number: u16,
    // pub cover: String,
}

#[tauri::command]
pub fn get_track_information(
    track_id: i64,
    global_state: State<Mutex<GlobalState>>,
) -> Result<TrackInformationResponse, String> {
    let state = global_state.lock().unwrap();

    let connnection = state
        .connection
        .lock()
        .map_err(|_| "Failed to acquire lock")?;

    let path = connnection
        .get_track_path_by_id(track_id, |row| {
            Ok(row.get_ref("path")?.as_str()?.to_string())
        })
        .unwrap();

    let tag = Tag::new()
        .read_from_path(path.clone())
        .map_err(|_| "Failed to read tag")?;

    let track_title = tag.title().unwrap().to_string();
    let album_title = tag.album_title().unwrap().to_string();
    let artist_name = tag.artist().unwrap().to_string();
    // let track_duration = tag.duration().unwrap();
    let track_number = tag.track_number().unwrap();
    // let cover = encode(tag.album_cover().unwrap().data);

    Ok(TrackInformationResponse {
        track_title,
        album_title,
        artist_name,
        // track_duration,
        track_number,
        // cover,
    })
}

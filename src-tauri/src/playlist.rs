use crate::{track::Track, GlobalState};
use include_sqlite_sql::{impl_sql, include_sql};

include_sql!("sql/Playlists.sql");
include_sql!("sql/PlaylistTracks.sql");

#[derive(Debug)]
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
pub fn get_all_playlists(state: &GlobalState) -> Result<Vec<Playlist>, String> {
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
pub fn create_playlist(name: &str, state: &GlobalState) -> Result<i64, String> {
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
    state: &GlobalState,
) -> Result<(), String> {
    let connnection = state.connection.lock().unwrap();

    connnection
        .add_track_to_playlist(playlist_id, track_id)
        .unwrap();

    Ok(())
}

#[tauri::command]
pub fn get_tracks_in_playlist(playlist_id: i64, state: &GlobalState) -> Result<Vec<Track>, String> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use std::sync::Mutex;

    #[test]
    fn can_get_all_playlists() {
        let state = GlobalState {
            connection: Mutex::new(Connection::open_in_memory().unwrap()),
        };

        {
            let connection = state.connection.lock().unwrap();

            connection.create_playlists_table().unwrap();

            connection
                .insert_playlist("Playlist #1", |_row| Ok(()))
                .unwrap();

            connection
                .insert_playlist("Playlist #2", |_row| Ok(()))
                .unwrap();

            connection
                .insert_playlist("Playlist #3", |_row| Ok(()))
                .unwrap();
        }

        let playlists = get_all_playlists(&state).unwrap();

        assert_eq!(
            format!("{:?}", playlists.to_vec()),
            r##"[Playlist { id: 1, name: "Playlist #1" }, Playlist { id: 2, name: "Playlist #2" }, Playlist { id: 3, name: "Playlist #3" }]"##,
        );
    }

    #[test]
    fn can_create_playlist() {
        let state = GlobalState {
            connection: Mutex::new(Connection::open_in_memory().unwrap()),
        };

        {
            let connection = state.connection.lock().unwrap();

            connection.create_playlists_table().unwrap();
        }

        create_playlist("Playlist #1", &state).unwrap();
        create_playlist("Playlist #2", &state).unwrap();
        create_playlist("Playlist #3", &state).unwrap();

        let playlists = get_all_playlists(&state).unwrap();

        assert_eq!(
            format!("{:?}", playlists.to_vec()),
            r##"[Playlist { id: 1, name: "Playlist #1" }, Playlist { id: 2, name: "Playlist #2" }, Playlist { id: 3, name: "Playlist #3" }]"##,
        );
    }
}

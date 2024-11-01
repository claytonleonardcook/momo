use crate::GlobalState;
use include_sqlite_sql::{impl_sql, include_sql};

include_sql!("sql/Tracks.sql");

#[derive(Debug)]
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
pub fn get_all_tracks(state: &GlobalState) -> Result<Vec<Track>, String> {
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
pub fn get_tracks_by_artist(artist_id: i64, state: &GlobalState) -> Result<Vec<Track>, String> {
    let connnection = state.connection.lock().unwrap();

    let tracks = &mut Vec::new();

    connnection
        .get_tracks_by_artist(artist_id, |row| {
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
pub fn get_tracks_by_album(album_id: i64, state: &GlobalState) -> Result<Vec<Track>, String> {
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

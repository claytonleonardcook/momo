use crate::GlobalState;
use include_sqlite_sql::{impl_sql, include_sql};
use std::sync::Mutex;
use tauri::State;

include_sql!("sql/Albums.sql");

#[derive(Debug)]
pub struct Album {
    pub id: i64,
    pub name: String,
    pub artist_name: String,
}

impl Album {
    pub fn new(id: i64, name: String, artist_name: String) -> Album {
        Album {
            id,
            name,
            artist_name,
        }
    }
}

impl std::clone::Clone for Album {
    fn clone(&self) -> Self {
        Album::new(self.id, self.name.clone(), self.artist_name.clone())
    }
}

#[tauri::command]
pub fn get_all_albums(global_state: &State<Mutex<GlobalState>>) -> Result<Vec<Album>, String> {
    let state = global_state.lock().unwrap();

    let connnection = state.connection.lock().unwrap();

    let albums = &mut Vec::new();

    connnection
        .get_all_albums(|row| {
            let id: i64 = row.get_ref("id")?.as_i64()?;
            let name: String = row.get_ref("name")?.as_str()?.to_string();
            let artist_name: String = row.get_ref("artist_name")?.as_str()?.to_string();

            albums.push(Album::new(id, name, artist_name));

            Ok(())
        })
        .unwrap();

    Ok(albums.to_vec())
}

#[tauri::command]
pub fn get_albums_by_artist(
    artist_name: &str,
    global_state: &State<Mutex<GlobalState>>,
) -> Result<Vec<Album>, String> {
    let state = global_state.lock().unwrap();

    let connnection = state.connection.lock().unwrap();

    let albums = &mut Vec::new();

    connnection
        .get_albums_by_artist(artist_name, |row| {
            let id: i64 = row.get_ref("id")?.as_i64()?;
            let name: String = row.get_ref("name")?.as_str()?.to_string();
            let artist_name: String = row.get_ref("artist_name")?.as_str()?.to_string();

            albums.push(Album::new(id, name, artist_name));

            Ok(())
        })
        .unwrap();

    Ok(albums.to_vec())
}

use crate::GlobalState;
use include_sqlite_sql::{impl_sql, include_sql};

include_sql!("sql/Albums.sql");

#[derive(Debug)]
pub struct Album {
    pub id: i64,
    pub name: String,
    pub artist_id: i64,
}

impl Album {
    pub fn new(id: i64, name: String, artist_id: i64) -> Album {
        Album {
            id,
            name,
            artist_id,
        }
    }
}

impl std::clone::Clone for Album {
    fn clone(&self) -> Self {
        Album::new(self.id, self.name.clone(), self.artist_id)
    }
}

#[tauri::command]
pub fn get_all_albums(state: &GlobalState) -> Result<Vec<Album>, String> {
    let connnection = state.connection.lock().unwrap();

    let albums = &mut Vec::new();

    connnection
        .get_all_albums(|row| {
            let id: i64 = row.get_ref("id")?.as_i64()?;
            let name: String = row.get_ref("name")?.as_str()?.to_string();
            let artist_id: i64 = row.get_ref("artist_id")?.as_i64()?;

            albums.push(Album::new(id, name, artist_id));

            Ok(())
        })
        .unwrap();

    Ok(albums.to_vec())
}

#[tauri::command]
pub fn get_albums_by_artist(artist_id: i64, state: &GlobalState) -> Result<Vec<Album>, String> {
    let connnection = state.connection.lock().unwrap();

    let albums = &mut Vec::new();

    connnection
        .get_albums_by_artist(artist_id, |row| {
            let id: i64 = row.get_ref("id")?.as_i64()?;
            let name: String = row.get_ref("name")?.as_str()?.to_string();
            let artist_id: i64 = row.get_ref("artist_id")?.as_i64()?;

            albums.push(Album::new(id, name, artist_id));

            Ok(())
        })
        .unwrap();

    Ok(albums.to_vec())
}

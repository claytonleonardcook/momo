use crate::GlobalState;
use include_sqlite_sql::{impl_sql, include_sql};

include_sql!("sql/Artists.sql");

#[derive(Debug)]
pub struct Artist {
    pub name: String,
}

impl Artist {
    pub fn new(name: String) -> Artist {
        Artist { name }
    }
}

impl std::clone::Clone for Artist {
    fn clone(&self) -> Self {
        Artist::new(self.name.clone())
    }
}

#[tauri::command]
pub fn get_all_artists(state: &GlobalState) -> Result<Vec<Artist>, String> {
    let connnection = state.connection.lock().unwrap();

    let artists = &mut Vec::new();

    connnection
        .get_all_artists(|row| {
            let name: String = row.get_ref("name")?.as_str()?.to_string();

            artists.push(Artist::new(name));

            Ok(())
        })
        .unwrap();

    Ok(artists.to_vec())
}

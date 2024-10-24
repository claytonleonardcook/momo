use include_sqlite_sql::{impl_sql, include_sql};

use crate::GlobalState;

include_sql!("sql/Artists.sql");

#[derive(Clone, Debug)]
pub struct Artist {
    #[allow(dead_code)]
    pub id: i64,
    #[allow(dead_code)]
    pub name: String,
}

impl Artist {
    pub fn new(id: i64, name: String) -> Artist {
        Artist { id, name }
    }
}

#[tauri::command]
pub fn get_all_artists(state: &GlobalState) -> Result<Vec<Artist>, String> {
    let connnection = state.connection.lock().unwrap();

    let artists = &mut Vec::new();

    connnection
        .get_all_artists(|row| {
            let id: i64 = row.get_ref("id")?.as_i64()?;
            let name: String = row.get_ref("name")?.as_str()?.to_string();

            artists.push(Artist::new(id, name));

            Ok(())
        })
        .unwrap();

    Ok(artists.to_vec())
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use rusqlite::Connection;

    use crate::ArtistsSql;

    use super::*;

    include_sql!("sql/Tracks.sql");

    #[test]
    fn can_get_all_artists() {
        let state = GlobalState {
            connection: Mutex::new(Connection::open_in_memory().unwrap()),
        };

        {
            let connection = state.connection.lock().unwrap();

            connection.create_artists_table().unwrap();

            connection.insert_artist("Alex G").unwrap();
            connection.insert_artist("Lizzy McAlpine").unwrap();
            connection.insert_artist("The Daughters of Eve").unwrap();
        }

        let artists = get_all_artists(&state).unwrap();

        assert_eq!(
            format!("{:?}", artists.to_vec()),
            r##"[Artist { id: 1, name: "Alex G" }, Artist { id: 2, name: "Lizzy McAlpine" }, Artist { id: 3, name: "The Daughters of Eve" }]"##,
        );
    }
}

use include_sqlite_sql::{impl_sql, include_sql};

use crate::GlobalState;

include_sql!("sql/Albums.sql");

#[derive(Clone, Debug)]
pub struct Album {
    #[allow(dead_code)]
    pub id: i64,
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use rusqlite::Connection;

    use crate::{AlbumsSql, ArtistsSql};

    use super::*;

    include_sql!("sql/Tracks.sql");

    #[test]
    fn can_get_all_albums() {
        let state = GlobalState {
            connection: Mutex::new(Connection::open_in_memory().unwrap()),
        };

        {
            let connection = state.connection.lock().unwrap();

            connection.create_artists_table().unwrap();

            connection.create_albums_table().unwrap();

            connection.create_tracks_table().unwrap();

            let aritst_id = connection.insert_artist("Alex G").unwrap();

            connection
                .insert_album("Rocket", aritst_id.try_into().unwrap())
                .unwrap();

            connection
                .insert_album("Trick", aritst_id.try_into().unwrap())
                .unwrap();
        }

        let albums = get_all_albums(&state).unwrap();

        assert_eq!(
            format!("{:?}", albums.to_vec()),
            r##"[Album { id: 1, name: "Rocket", artist_id: 1 }, Album { id: 2, name: "Trick", artist_id: 1 }]"##,
        );
    }
}

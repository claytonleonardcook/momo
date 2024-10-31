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

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use std::sync::Mutex;

    include_sql!("sql/Albums.sql");
    include_sql!("sql/Artists.sql");

    #[test]
    fn can_get_all_tracks() {
        let state = GlobalState {
            connection: Mutex::new(Connection::open_in_memory().unwrap()),
        };

        {
            let connection = state.connection.lock().unwrap();

            connection.create_artists_table().unwrap();
            connection.create_albums_table().unwrap();
            connection.create_tracks_table().unwrap();

            let artist_id = connection
                .insert_artist("Alex G", |row| Ok(row.get_ref("id")?.as_i64()?))
                .unwrap();

            let album_id = connection
                .insert_album("Rocket", artist_id, |row| {
                    Ok(row.get_ref("id")?.as_i64()?)
                })
                .unwrap();

            connection
                .insert_track("Bobby", "./hello", album_id, |_row| Ok(()))
                .unwrap();

            connection
                .insert_track("Proud", "./world", album_id, |_row| Ok(()))
                .unwrap();
        }

        let tracks = get_all_tracks(&state).unwrap();

        assert_eq!(
            format!("{:?}", tracks.to_vec()),
            r##"[Track { id: 1, name: "Bobby", path: "./hello", album_id: 1 }, Track { id: 2, name: "Proud", path: "./world", album_id: 1 }]"##,
        );
    }

    #[test]
    fn can_get_tracks_by_artist() {
        let state = GlobalState {
            connection: Mutex::new(Connection::open_in_memory().unwrap()),
        };

        {
            let connection = state.connection.lock().unwrap();

            connection.create_artists_table().unwrap();
            connection.create_albums_table().unwrap();
            connection.create_tracks_table().unwrap();

            let artist_id = connection
                .insert_artist("Alex G", |row| Ok(row.get_ref("id")?.as_i64()?))
                .unwrap();

            {
                let album_id = connection
                    .insert_album("Rocket", artist_id, |row| {
                        Ok(row.get_ref("id")?.as_i64()?)
                    })
                    .unwrap();

                connection
                    .insert_track("Bobby", "./hello", album_id, |_row| Ok(()))
                    .unwrap();

                connection
                    .insert_track("Proud", "./world", album_id, |_row| Ok(()))
                    .unwrap();
            }

            {
                let album_id = connection
                    .insert_album("Trick", artist_id, |row| Ok(row.get_ref("id")?.as_i64()?))
                    .unwrap();

                connection
                    .insert_track("Memory", "./memory", album_id, |_row| Ok(()))
                    .unwrap();
            }
        }

        let tracks = get_tracks_by_artist(1, &state).unwrap();

        assert_eq!(
            format!("{:?}", tracks.to_vec()),
            r##"[Track { id: 1, name: "Bobby", path: "./hello", album_id: 1 }, Track { id: 2, name: "Proud", path: "./world", album_id: 1 }, Track { id: 3, name: "Memory", path: "./memory", album_id: 2 }]"##,
        );
    }
}

use crate::GlobalState;

use crate::TracksSql;

#[derive(Debug)]
pub struct Track {
    #[allow(dead_code)]
    pub id: i64,
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
    pub path: String,
    #[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use rusqlite::Connection;

    use crate::{AlbumsSql, ArtistsSql};

    use super::*;

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

            let aritst_id = connection.insert_artist("Alex G").unwrap();

            let album_id = connection
                .insert_album("Rocket", aritst_id.try_into().unwrap())
                .unwrap();

            connection
                .insert_track("Bobby", "./hello", album_id.try_into().unwrap())
                .unwrap();

            connection
                .insert_track("Proud", "./world", album_id.try_into().unwrap())
                .unwrap();
        }

        let tracks = get_all_tracks(&state).unwrap();

        assert_eq!(
            format!("{:?}", tracks.to_vec()),
            r##"[Track { id: 1, name: "Bobby", path: "./hello", album_id: 1 }, Track { id: 2, name: "Proud", path: "./world", album_id: 1 }]"##,
        );
    }
}

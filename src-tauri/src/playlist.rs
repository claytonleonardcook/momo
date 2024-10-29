use crate::{GlobalState, PlaylistsSql};

#[derive(Debug)]
pub struct Playlist {
    #[allow(dead_code)]
    pub id: i64,
    #[allow(dead_code)]
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
pub fn create_playlists(name: &str, state: &GlobalState) -> Result<(), String> {
    let connnection = state.connection.lock().unwrap();

    connnection.insert_playlist(name).unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use rusqlite::Connection;

    use super::*;

    #[test]
    fn can_get_all_playlists() {
        let state = GlobalState {
            connection: Mutex::new(Connection::open_in_memory().unwrap()),
        };

        {
            let connection = state.connection.lock().unwrap();

            connection.create_playlists_table().unwrap();

            connection.insert_playlist("Playlist #1").unwrap();

            connection.insert_playlist("Playlist #2").unwrap();

            connection.insert_playlist("Playlist #3").unwrap();
        }

        let playlists = get_all_playlists(&state).unwrap();

        assert_eq!(
            format!("{:?}", playlists.to_vec()),
            r##"[Playlist { id: 1, name: "Playlist #1" }, Playlist { id: 2, name: "Playlist #2" }, Playlist { id: 3, name: "Playlist #3" }]"##,
        );
    }

    #[test]
    fn can_create_playlists() {
        let state = GlobalState {
            connection: Mutex::new(Connection::open_in_memory().unwrap()),
        };

        {
            let connection = state.connection.lock().unwrap();

            connection.create_playlists_table().unwrap();
        }

        create_playlists("Playlist #1", &state).unwrap();
        create_playlists("Playlist #2", &state).unwrap();
        create_playlists("Playlist #3", &state).unwrap();

        let playlists = get_all_playlists(&state).unwrap();

        assert_eq!(
            format!("{:?}", playlists.to_vec()),
            r##"[Playlist { id: 1, name: "Playlist #1" }, Playlist { id: 2, name: "Playlist #2" }, Playlist { id: 3, name: "Playlist #3" }]"##,
        );
    }
}

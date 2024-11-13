use crate::GlobalState;
use audiotags::Tag;
use include_sqlite_sql::{impl_sql, include_sql};
use std::{
    path::{Path, PathBuf},
    sync::Mutex,
};
use tauri::State;

include_sql!("sql/Tracks.sql");
include_sql!("sql/Albums.sql");
include_sql!("sql/Artists.sql");
include_sql!("sql/Playlists.sql");
include_sql!("sql/PlaylistTracks.sql");

pub fn collect_mp3_files(dir: &Path, mp3_files: &mut Vec<PathBuf>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    // If the entry is a directory, call the function recursively
                    collect_mp3_files(&path, mp3_files);
                } else if let Some(ext) = path.extension() {
                    if ext == "mp3" {
                        // If the entry is a .mp3 file, add it to the vector
                        mp3_files.push(path);
                    }
                }
            }
        }
    }
}

pub fn insert_tracks_into_database(global_state: &State<Mutex<GlobalState>>, paths: Vec<PathBuf>) {
    let state = global_state.lock().unwrap();

    for path in paths {
        let connection = state.connection.lock().unwrap();

        let tag = Tag::new().read_from_path(path.clone()).unwrap();

        let artist_name = tag.artist().unwrap();

        match connection.insert_artist(artist_name, |_row| Ok(())) {
            Ok(_) => (),
            Err(error) => println!("{}", error),
        }

        let album_name = tag.album_title().unwrap();

        let album_id = connection
            .insert_album(album_name, artist_name, |row| {
                Ok(row.get_ref("id").unwrap().as_i64().unwrap())
            })
            .unwrap();

        if let Some(track_name) = tag.title() {
            if let Some(track_path) = path.as_os_str().to_str() {
                connection
                    .insert_track(track_name, track_path, album_id, |_row| Ok(()))
                    .unwrap();
            }
        }
    }
}

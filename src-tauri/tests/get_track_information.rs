use audiotags::Tag;
use include_sqlite_sql::{impl_sql, include_sql};
use momo::{track::get_track_information, GlobalState};
use rusqlite::Connection;
use std::{
    path::{Path, PathBuf},
    sync::Mutex,
};

mod common;

include_sql!("sql/Tracks.sql");
include_sql!("sql/Albums.sql");
include_sql!("sql/Artists.sql");
include_sql!("sql/Playlists.sql");
include_sql!("sql/PlaylistTracks.sql");

fn collect_mp3_files(dir: &Path, mp3_files: &mut Vec<PathBuf>) {
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

#[test]
fn can_get_track_information() {
    let state = GlobalState {
        connection: Mutex::new(Connection::open_in_memory().unwrap()),
    };

    common::create_tables(&state).unwrap();

    {
        let mut paths = Vec::new();

        collect_mp3_files(Path::new("../public"), &mut paths);

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

    common::print_all_tracks_by_artist("Cat", &state).unwrap();

    let track_information = get_track_information(1, &state).unwrap();

    assert_eq!(track_information.track_title, "Paw");
    assert_eq!(track_information.album_title, "Purr");
    assert_eq!(track_information.artist_name, "Cat");
    assert_eq!(track_information.track_number, 2);
}

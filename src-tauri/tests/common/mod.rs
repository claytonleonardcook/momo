use std::sync::Mutex;

use include_sqlite_sql::{impl_sql, include_sql};
use momo::GlobalState;
use tauri::State;

include_sql!("sql/Tracks.sql");
include_sql!("sql/Albums.sql");
include_sql!("sql/Artists.sql");
include_sql!("sql/Playlists.sql");
include_sql!("sql/PlaylistTracks.sql");

pub fn create_tables(global_state: &State<Mutex<GlobalState>>) -> Result<(), ()> {
    let state = global_state.lock().unwrap();
    let connection = state.connection.lock().unwrap();

    connection.create_artists_table().unwrap();
    connection.create_albums_table().unwrap();
    connection.create_tracks_table().unwrap();
    connection.create_playlists_table().unwrap();
    connection.create_playlist_tracks_table().unwrap();

    Ok(())
}

// TODO: fix dead code warning
#[allow(dead_code)]
pub fn create_artist(name: &str, global_state: &State<Mutex<GlobalState>>) -> Result<String, ()> {
    let state = global_state.lock().unwrap();
    let connection = state.connection.lock().unwrap();

    Ok(connection
        .insert_artist(name, |row| Ok(row.get_ref("name")?.as_str()?.to_string()))
        .unwrap())
}

// TODO: fix dead code warning
#[allow(dead_code)]
pub fn print_all_tracks_by_artist(
    artist_name: &str,
    global_state: &State<Mutex<GlobalState>>,
) -> Result<(), ()> {
    let state = global_state.lock().unwrap();
    let connection = state.connection.lock().unwrap();

    println!(
        "{0: <10} | {1: <10} | {2: <40} | {3: <10}",
        "track name", "album name", "track path", "artist name"
    );

    connection
        .get_tracks_by_artist(artist_name, |row| {
            let name = row.get_ref("name")?.as_str()?;
            let album_name = row.get_ref("album_name")?.as_str()?;
            let path = row.get_ref("path")?.as_str()?;
            let artist_name = row.get_ref("artist_name")?.as_str()?;

            println!(
                "{0: <10} | {1: <10} | {2: <40} | {3: <10}",
                name, album_name, path, artist_name
            );
            Ok(())
        })
        .unwrap();

    println!();

    Ok(())
}

// TODO: fix dead code warning
#[allow(dead_code)]
pub fn create_album(
    name: &str,
    artist_name: &str,
    global_state: &State<Mutex<GlobalState>>,
) -> Result<i64, ()> {
    let state = global_state.lock().unwrap();
    let connection = state.connection.lock().unwrap();

    Ok(connection
        .insert_album(name, artist_name, |row| Ok(row.get_ref("id")?.as_i64()?))
        .unwrap())
}

// TODO: fix dead code warning
#[allow(dead_code)]
pub fn create_track(
    name: &str,
    path: &str,
    album_id: i64,
    global_state: &State<Mutex<GlobalState>>,
) -> Result<i64, ()> {
    let state = global_state.lock().unwrap();
    let connection = state.connection.lock().unwrap();

    Ok(connection
        .insert_track(name, &path, album_id, |row| {
            Ok(row.get_ref("id")?.as_i64()?)
        })
        .unwrap())
}

// TODO: fix dead code warning
#[allow(dead_code)]
pub fn create_playlist(name: &str, global_state: &State<Mutex<GlobalState>>) -> Result<i64, ()> {
    let state = global_state.lock().unwrap();
    let connection = state.connection.lock().unwrap();

    Ok(connection
        .insert_playlist(name, |row| Ok(row.get_ref("id")?.as_i64()?))
        .unwrap())
}

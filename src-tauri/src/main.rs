// ! Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use include_sqlite_sql::{impl_sql, include_sql};
use momo::GlobalState;
use rusqlite::{Connection, Result};
use std::sync::Mutex;
use tauri::Manager;

include_sql!("sql/Tracks.sql");
include_sql!("sql/Albums.sql");
include_sql!("sql/Artists.sql");
include_sql!("sql/Playlists.sql");
include_sql!("sql/PlaylistTracks.sql");

#[tauri::command]
fn greet(name: &str) -> Result<String, String> {
    if name.is_empty() {
        return Err("Provide a valid name!".into());
    }

    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}

fn main() -> Result<()> {
    tauri::Builder::default()
        .setup(|app| {
            let connection = Connection::open_in_memory().unwrap();

            connection.create_artists_table().unwrap();
            connection.create_albums_table().unwrap();
            connection.create_tracks_table().unwrap();
            connection.create_playlist_tracks_table().unwrap();

            connection.insert_artist("Alex G", |_row| Ok(())).unwrap();

            app.manage(GlobalState {
                connection: Mutex::new(connection),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

// ! Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use include_sqlite_sql::{impl_sql, include_sql};
use rusqlite::{Connection, Result};
use tauri::Manager;

include_sql!("sql/Artists.sql");
include_sql!("sql/Albums.sql");
include_sql!("sql/Tracks.sql");
include_sql!("sql/Playlists.sql");
include_sql!("sql/PlaylistTracks.sql");

struct GlobalState {
    conn: Mutex<Connection>,
}

#[tauri::command]
fn greet(name: &str, state: tauri::State<GlobalState>) -> Result<String, String> {
    if name.is_empty() {
        return Err("Provide a valid name!".into());
    }

    let conn = state.conn.lock().unwrap();

    conn.get_all_artists(|row| {
        let name: &str = row.get_ref("name")?.as_str()?;
        let artist_id: i64 = row.get_ref("id")?.as_i64()?;

        println!("{}", name);

        conn.insert_album("Rocket", artist_id).unwrap();

        Ok(())
    })
    .unwrap();

    conn.get_all_albums(|row| {
        let name: &str = row.get_ref("name")?.as_str()?;

        println!("{}", name);

        Ok(())
    })
    .unwrap();

    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}

fn main() -> Result<()> {
    tauri::Builder::default()
        .setup(|app| {
            let conn = Connection::open_in_memory().unwrap();

            conn.create_artists_table().unwrap();
            conn.create_albums_table().unwrap();
            conn.create_tracks_table().unwrap();

            conn.insert_artist("Alex G").unwrap();

            app.manage(GlobalState {
                conn: Mutex::new(conn),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

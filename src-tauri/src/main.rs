// ! Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use include_sqlite_sql::{impl_sql, include_sql};
use momo::{
    track,
    utilities::{collect_mp3_files, insert_tracks_into_database},
    GlobalState,
};
use rodio::{source::SineWave, OutputStream, Sink, Source};
use rusqlite::Result;
use std::{path::Path, sync::Mutex, time::Duration};
use tauri::{Manager, State};

include_sql!("sql/Tracks.sql");
include_sql!("sql/Albums.sql");
include_sql!("sql/Artists.sql");
include_sql!("sql/Playlists.sql");
include_sql!("sql/PlaylistTracks.sql");

#[tauri::command]
fn pause(global_state: State<Mutex<GlobalState>>) -> Result<(), String> {
    let state = global_state.lock().unwrap();

    let sink = state.sink.as_ref().unwrap();
    sink.pause();

    Ok(())
}

#[tauri::command]
fn play(global_state: State<Mutex<GlobalState>>) -> Result<(), String> {
    let state = global_state.lock().unwrap();

    let sink = state.sink.as_ref().unwrap();
    sink.play();

    Ok(())
}

#[tauri::command]
fn set_volume(volume: f32, global_state: State<Mutex<GlobalState>>) -> Result<(), String> {
    let state = global_state.lock().unwrap();

    let sink = state.sink.as_ref().unwrap();
    sink.set_volume(volume);

    Ok(())
}

fn main() -> Result<()> {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    tauri::Builder::default()
        .manage(Mutex::new(GlobalState::new(sink)))
        .setup(|app| {
            let global_state: State<Mutex<GlobalState>> = app.state();

            {
                let state = global_state.lock().unwrap();
                let connection = state.connection.lock().unwrap();

                connection.create_artists_table().unwrap();
                connection.create_albums_table().unwrap();
                connection.create_tracks_table().unwrap();
                connection.create_playlist_tracks_table().unwrap();
            }

            {
                let mut paths = Vec::new();

                collect_mp3_files(Path::new("../public"), &mut paths);
                insert_tracks_into_database(&global_state, paths);
            }

            {
                let state = global_state.lock().unwrap();
                let connection = state.connection.lock().unwrap();

                println!(
                    "{0: <10} | {1: <10} | {2: <40} | {3: <10}",
                    "track name", "album name", "track path", "artist name"
                );

                connection
                    .get_tracks_by_artist("Cat", |row| {
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
            }

            {
                let state = global_state.lock().unwrap();

                let source = SineWave::new(440.0)
                    .take_duration(Duration::from_secs(30))
                    .amplify(0.20);

                let sink = state.sink.as_ref().unwrap();

                sink.append(source);
                sink.pause();
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            pause,
            play,
            set_volume,
            track::get_all_tracks
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

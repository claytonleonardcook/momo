// ! Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use include_sqlite_sql::{impl_sql, include_sql};
use momo::{
    album, artist, playlist, track,
    utilities::{collect_mp3_files, insert_tracks_into_database},
    GlobalState,
};
use rodio::{Decoder, OutputStream, Sink};
use rusqlite::Result;
use std::{fs::File, io::BufReader, path::Path, sync::Mutex};
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

#[tauri::command]
fn get_track_position(global_state: State<Mutex<GlobalState>>) -> Result<u64, String> {
    let state = global_state.lock().unwrap();

    let sink = state.sink.as_ref().unwrap();
    let track_position = sink.get_pos().as_secs();

    Ok(track_position)
}

fn main() -> Result<()> {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    tauri::Builder::default()
        .manage(Mutex::new(GlobalState::new(sink)))
        .setup(|app| {
            {
                let global_state = app.state::<Mutex<GlobalState>>();
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
                insert_tracks_into_database(app.state(), paths);
            }

            {
                let global_state = app.state::<Mutex<GlobalState>>();
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
                let global_state = app.state::<Mutex<GlobalState>>();
                let state = global_state.lock().unwrap();

                let file = BufReader::new(File::open("/home/clay/Music/Pinegrove - Everything So Far/Pinegrove - Everything So Far - 07 Need 2.mp3").unwrap());
                let source = Decoder::new(file).unwrap();

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
            get_track_position,
            track::get_all_tracks,
            track::get_tracks_by_artist,
            track::get_tracks_by_album,
            track::get_track_information,
            album::get_all_albums,
            album::get_albums_by_artist,
            artist::get_all_artists,
            playlist::get_all_playlists,
            playlist::add_track_to_playlist,
            playlist::get_tracks_in_playlist
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

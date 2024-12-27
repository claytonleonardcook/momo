use crate::utilities::{collect_mp3_files, insert_tracks_into_database};
use include_sqlite_sql::{impl_sql, include_sql};
use queues::Queue;
use rodio::{Decoder, Source};
use rodio::{OutputStream, Sink};
use rusqlite::Connection;
use rusqlite::Result;
use serde::Serialize;
use std::sync::Mutex;
use std::time::Duration;
use tauri::AppHandle;
use tauri::{Manager, State};

pub mod album;
pub mod artist;
pub mod playlist;
pub mod track;
pub mod utilities;

include_sql!("sql/Tracks.sql");
include_sql!("sql/Albums.sql");
include_sql!("sql/Artists.sql");
include_sql!("sql/Playlists.sql");
include_sql!("sql/PlaylistTracks.sql");

#[derive(Debug, Serialize)]
struct TrackPositionPayload {
    position: u64,
    total_duration: u64,
}

pub struct GlobalState {
    pub connection: std::sync::Mutex<rusqlite::Connection>,
    pub sink: Option<rodio::Sink>,
    pub queue: Queue<String>,
    pub music_folder_paths: Vec<String>,
    pub current_track_duration: u64,
}

impl GlobalState {
    pub fn new(sink: rodio::Sink) -> Self {
        GlobalState {
            connection: Mutex::new(Connection::open_in_memory().unwrap()),
            sink: Some(sink),
            queue: Queue::new(),
            music_folder_paths: Vec::new(),
            current_track_duration: 0,
        }
    }

    pub fn set_current_track_duration(&mut self, duration: Duration) {
        self.current_track_duration = duration.as_secs();
    }

    pub fn add_music_folder_path(&mut self, path: &str) {
        self.music_folder_paths.push(path.to_string());
    }
}

impl Default for GlobalState {
    fn default() -> Self {
        GlobalState {
            connection: Mutex::new(Connection::open_in_memory().unwrap()),
            sink: Option::None,
            queue: Queue::new(),
            music_folder_paths: Vec::new(),
            current_track_duration: 0,
        }
    }
}

#[tauri::command]
fn set_volume(volume: f32, global_state: State<Mutex<GlobalState>>) -> Result<(), &str> {
    let state = global_state.lock().unwrap();

    if let Some(sink) = &state.sink {
        sink.set_volume(volume);
    } else {
        return Err("Couldn't set the volume!");
    }

    Ok(())
}

#[tauri::command]
fn play_track(track_id: i64, global_state: State<Mutex<GlobalState>>) -> Result<(), &str> {
    let mut state = global_state.lock().unwrap();

    if let Some(sink) = &state.sink {
        sink.clear();

        let path = {
            let connection = state.connection.lock().unwrap();
            connection
                .get_track_path_by_id(track_id, |row| {
                    Ok(row.get_ref("path")?.as_str()?.to_string())
                })
                .unwrap()
        };

        let track = std::io::BufReader::new(std::fs::File::open(path).unwrap());
        let source = Decoder::new(track).unwrap();

        let total_duration = source.total_duration();

        sink.append(source);
        sink.play();

        if let Some(duration) = total_duration {
            state.set_current_track_duration(duration);
        };
    } else {
        return Err("Couldn't play track!");
    }

    Ok(())
}

#[tauri::command]
fn get_music_folder_paths(global_state: State<Mutex<GlobalState>>) -> Result<Vec<String>, String> {
    let state = global_state.lock().unwrap();

    Ok(state.music_folder_paths.clone())
}

#[tauri::command]
fn get_track_position(
    global_state: State<Mutex<GlobalState>>,
) -> Result<TrackPositionPayload, &str> {
    let state = global_state.lock().unwrap();

    if let Some(sink) = &state.sink {
        let position = sink.get_pos().as_secs();
        let total_duration = state.current_track_duration;

        return Ok(TrackPositionPayload {
            position,
            total_duration,
        });
    }

    Err("Couldn't get track position!")
}

#[tauri::command]
fn add_music_folder_path(
    path: &str,
    global_state: State<Mutex<GlobalState>>,
) -> Result<Vec<String>, String> {
    let mut state = global_state.lock().unwrap();

    state.add_music_folder_path(path);

    Ok(state.music_folder_paths.clone())
}

#[tauri::command]
fn scan_directories(app: AppHandle, global_state: State<Mutex<GlobalState>>) -> Result<(), String> {
    let mut paths = Vec::new();

    {
        let state = global_state.lock().unwrap();
        let music_folder_paths = state.music_folder_paths.clone();

        collect_mp3_files(music_folder_paths, &mut paths);
    }

    insert_tracks_into_database(app.app_handle(), global_state, paths);

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
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
                let global_state = app.state::<Mutex<GlobalState>>();
                let mut state = global_state.lock().unwrap();

                match app.path().audio_dir() {
                    Ok(path) => {
                        if let Some(path_str) = path.to_str() {
                            state.add_music_folder_path(path_str);
                        }
                    }
                    Err(_) => eprint!("Couldn't get main music directory!"),
                }
            }
            {
                let global_state = app.state::<Mutex<GlobalState>>();
                let app_handle = app.app_handle();
                match scan_directories(app_handle.clone(), global_state) {
                    Ok(_) => {}
                    Err(_) => eprint!("Couldn't scan music directories!"),
                }
            }

            // {
            //     let global_state = app.state::<Mutex<GlobalState>>();
            //     let state = global_state.lock().unwrap();
            //     let connection = state.connection.lock().unwrap();

            //     println!(
            //         "{0: <10} | {1: <10} | {2: <40} | {3: <10}",
            //         "track name", "album name", "track path", "artist name"
            //     );

            //     connection
            //         .get_tracks_by_artist("Cat", |row| {
            //             let name = row.get_ref("name")?.as_str()?;
            //             let album_name = row.get_ref("album_name")?.as_str()?;
            //             let path = row.get_ref("path")?.as_str()?;
            //             let artist_name = row.get_ref("artist_name")?.as_str()?;

            //             println!(
            //                 "{0: <10} | {1: <10} | {2: <40} | {3: <10}",
            //                 name, album_name, path, artist_name
            //             );
            //             Ok(())
            //         })
            //         .unwrap();

            //     println!();
            // }

            // {
            //     let global_state = app.state::<Mutex<GlobalState>>();
            //     let state = global_state.lock().unwrap();

            //     let file = BufReader::new(File::open("/home/clay/Music/Pinegrove - Everything So Far/Pinegrove - Everything So Far - 07 Need 2.mp3").unwrap());
            //     let source = Decoder::new(file).unwrap();

            //     let sink = state.sink.as_ref().unwrap();

            //     sink.append(source);
            //     sink.pause();
            // }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_music_folder_paths,
            add_music_folder_path,
            scan_directories,
            play_track,
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
}

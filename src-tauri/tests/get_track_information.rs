use include_sqlite_sql::{impl_sql, include_sql};
use momo::{
    track::get_track_information,
    utilities::{collect_mp3_files, insert_tracks_into_database},
    GlobalState,
};
use std::{path::Path, sync::Mutex};
use tauri::Manager;
use tauri::State;

mod common;

include_sql!("sql/Tracks.sql");
include_sql!("sql/Albums.sql");
include_sql!("sql/Artists.sql");
include_sql!("sql/Playlists.sql");
include_sql!("sql/PlaylistTracks.sql");

#[test]
fn can_get_track_information() {
    tauri::test::mock_builder()
        .manage(Mutex::new(GlobalState::default()))
        .setup(|app| {
            let state: State<Mutex<GlobalState>> = app.state();

            common::create_tables(&state).unwrap();

            {
                let mut paths = Vec::new();

                collect_mp3_files(Path::new("../public"), &mut paths);

                insert_tracks_into_database(&state, paths);
            }

            common::print_all_tracks_by_artist("Cat", &state).unwrap();

            let track_information = get_track_information(1, &state).unwrap();

            assert_eq!(track_information.track_title, "Paw");
            assert_eq!(track_information.album_title, "Purr");
            assert_eq!(track_information.artist_name, "Cat");
            assert_eq!(track_information.track_number, 2);
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("Failed to run test app!");
}

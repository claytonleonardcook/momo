use include_sqlite_sql::{impl_sql, include_sql};
use momo_lib::{
    track::get_track_information,
    utilities::{collect_mp3_files, insert_tracks_into_database},
    GlobalState,
};
use std::sync::Mutex;
use tauri::Manager;

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
            common::create_tables(app.state()).unwrap();

            {
                let mut paths = Vec::new();

                collect_mp3_files(vec!["../public".to_string()], &mut paths);

                insert_tracks_into_database(app.handle(), app.state(), paths);
            }

            common::print_all_tracks_by_artist("Cat", app.state()).unwrap();

            let track_information = get_track_information(1, app.state()).unwrap();

            assert_eq!(track_information.track_title, "Paw");
            assert_eq!(track_information.album_title, "Purr");
            assert_eq!(track_information.artist_name, "Cat");
            assert_eq!(track_information.track_number, 2);
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("Failed to run test app!");
}

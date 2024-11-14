use include_sqlite_sql::{impl_sql, include_sql};
use momo_lib::{playlist::create_playlist, playlist::get_all_playlists, GlobalState};
use std::sync::Mutex;
use tauri::Manager;

mod common;

include_sql!("sql/Tracks.sql");
include_sql!("sql/Albums.sql");
include_sql!("sql/Artists.sql");
include_sql!("sql/Playlists.sql");
include_sql!("sql/PlaylistTracks.sql");

#[test]
fn can_create_playlist() {
    tauri::test::mock_builder()
        .manage(Mutex::new(GlobalState::default()))
        .setup(|app| {
            common::create_tables(app.state()).unwrap();

            create_playlist("Playlist #1", app.state()).unwrap();
            create_playlist("Playlist #2", app.state()).unwrap();
            create_playlist("Playlist #3", app.state()).unwrap();

            let playlists = get_all_playlists(app.state()).unwrap();

            assert_eq!(playlists.get(0).unwrap().id, 1);
            assert_eq!(playlists.get(0).unwrap().name, "Playlist #1");

            assert_eq!(playlists.get(1).unwrap().id, 2);
            assert_eq!(playlists.get(1).unwrap().name, "Playlist #2");

            assert_eq!(playlists.get(2).unwrap().id, 3);
            assert_eq!(playlists.get(2).unwrap().name, "Playlist #3");

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("Failed to run test app!");
}

use include_sqlite_sql::{impl_sql, include_sql};
use momo_lib::{album::get_all_albums, GlobalState};
use std::sync::Mutex;
use tauri::Manager;

mod common;

include_sql!("sql/Tracks.sql");
include_sql!("sql/Albums.sql");
include_sql!("sql/Artists.sql");
include_sql!("sql/Playlists.sql");
include_sql!("sql/PlaylistTracks.sql");

#[test]
fn can_get_all_albums() {
    tauri::test::mock_builder()
        .manage(Mutex::new(GlobalState::default()))
        .setup(|app| {
            common::create_tables(app.state()).unwrap();

            {
                let artist_name = common::create_artist("Alex G", app.state()).unwrap();

                common::create_album("Rocket", artist_name.as_str(), app.state()).unwrap();
                common::create_album("Trick", artist_name.as_str(), app.state()).unwrap();
            }

            let albums = get_all_albums(app.state()).unwrap();

            assert_eq!(albums.get(0).unwrap().id, 1);
            assert_eq!(albums.get(0).unwrap().name, "Rocket");

            assert_eq!(albums.get(1).unwrap().id, 2);
            assert_eq!(albums.get(1).unwrap().name, "Trick");
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("Failed to run test app!");
}

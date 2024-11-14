use include_sqlite_sql::{impl_sql, include_sql};
use momo_lib::{
    playlist::{add_track_to_playlist, create_playlist, get_tracks_in_playlist},
    track::get_all_tracks,
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
fn can_add_track_to_playlist() {
    tauri::test::mock_builder()
        .manage(Mutex::new(GlobalState::default()))
        .setup(|app| {
            common::create_tables(app.state()).unwrap();

            let playlist_id = create_playlist("Playlist #1", app.state()).unwrap();

            {
                let artist_name = common::create_artist("Alex G", app.state()).unwrap();

                let album_id =
                    common::create_album("Rocket", artist_name.as_str(), app.state()).unwrap();

                for track in vec!["Poison Root", "Proud", "County", "Bobby"]
                    .iter()
                    .copied()
                {
                    let name = track;
                    let path = format!("./{}", track.to_ascii_lowercase().trim());

                    common::create_track(name, &path, album_id, app.state()).unwrap();
                }
            }

            {
                let tracks = get_all_tracks(app.state()).unwrap();

                for track in tracks.iter() {
                    if track.id % 2 == 0 {
                        add_track_to_playlist(playlist_id, track.id, app.state()).unwrap();
                    }
                }
            }

            let tracks = get_tracks_in_playlist(playlist_id, app.state()).unwrap();

            assert_eq!(tracks.get(0).unwrap().id, 2);
            assert_eq!(tracks.get(0).unwrap().name, "Proud");

            assert_eq!(tracks.get(1).unwrap().id, 4);
            assert_eq!(tracks.get(1).unwrap().name, "Bobby");

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("Failed to run test app!");
}

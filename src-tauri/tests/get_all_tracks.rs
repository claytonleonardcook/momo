use include_sqlite_sql::{impl_sql, include_sql};
use momo::{track::get_all_tracks, GlobalState};
use rusqlite::Connection;
use std::sync::Mutex;

mod common;

include_sql!("sql/Tracks.sql");
include_sql!("sql/Albums.sql");
include_sql!("sql/Artists.sql");
include_sql!("sql/Playlists.sql");
include_sql!("sql/PlaylistTracks.sql");

#[test]
fn can_get_all_tracks() {
    let state = GlobalState {
        connection: Mutex::new(Connection::open_in_memory().unwrap()),
    };

    common::create_tables(&state).unwrap();

    {
        let artist_id = common::create_artist("Alex G", &state).unwrap();

        let album_id = common::create_album("Rocket", artist_id, &state).unwrap();

        common::create_track("Bobby", "./bobby", album_id, &state).unwrap();
        common::create_track("Proud", "./proud", album_id, &state).unwrap();
    }

    let tracks = get_all_tracks(&state).unwrap();

    assert_eq!(tracks.get(0).unwrap().id, 1);
    assert_eq!(tracks.get(0).unwrap().name, "Bobby");

    assert_eq!(tracks.get(1).unwrap().id, 2);
    assert_eq!(tracks.get(1).unwrap().name, "Proud");
}

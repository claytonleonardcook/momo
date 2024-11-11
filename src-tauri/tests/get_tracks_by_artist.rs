use include_sqlite_sql::{impl_sql, include_sql};
use momo::{track::get_tracks_by_artist, GlobalState};
use rusqlite::Connection;
use std::sync::Mutex;

mod common;

include_sql!("sql/Tracks.sql");
include_sql!("sql/Albums.sql");
include_sql!("sql/Artists.sql");
include_sql!("sql/Playlists.sql");
include_sql!("sql/PlaylistTracks.sql");

#[test]
fn can_get_tracks_by_artist() {
    let state = GlobalState {
        connection: Mutex::new(Connection::open_in_memory().unwrap()),
    };

    common::create_tables(&state).unwrap();

    {
        let artist_name = common::create_artist("Alex G", &state).unwrap();

        {
            let album_id = common::create_album("Rocket", artist_name.as_str(), &state).unwrap();

            common::create_track("Bobby", "./bobby", album_id, &state).unwrap();
            common::create_track("Proud", "./proud", album_id, &state).unwrap();
        }

        {
            let album_id = common::create_album("Trick", artist_name.as_str(), &state).unwrap();

            common::create_track("Memory", "./memory", album_id, &state).unwrap();
        }
    }

    let tracks = get_tracks_by_artist("Alex G", &state).unwrap();

    assert_eq!(tracks.get(0).unwrap().id, 1);
    assert_eq!(tracks.get(0).unwrap().name, "Bobby");

    assert_eq!(tracks.get(1).unwrap().id, 2);
    assert_eq!(tracks.get(1).unwrap().name, "Proud");

    assert_eq!(tracks.get(2).unwrap().id, 3);
    assert_eq!(tracks.get(2).unwrap().name, "Memory");
}

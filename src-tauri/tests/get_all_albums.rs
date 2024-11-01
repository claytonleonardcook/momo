use include_sqlite_sql::{impl_sql, include_sql};
use momo::{album::get_all_albums, GlobalState};
use rusqlite::Connection;
use std::sync::Mutex;

mod common;

include_sql!("sql/Tracks.sql");
include_sql!("sql/Albums.sql");
include_sql!("sql/Artists.sql");
include_sql!("sql/Playlists.sql");
include_sql!("sql/PlaylistTracks.sql");

#[test]
fn can_get_all_albums() {
    let state = GlobalState {
        connection: Mutex::new(Connection::open_in_memory().unwrap()),
    };

    common::create_tables(&state).unwrap();

    {
        let artist_id = common::create_artist("Alex G", &state).unwrap();

        common::create_album("Rocket", artist_id, &state).unwrap();
        common::create_album("Trick", artist_id, &state).unwrap();
    }

    let albums = get_all_albums(&state).unwrap();

    assert_eq!(albums.get(0).unwrap().id, 1);
    assert_eq!(albums.get(0).unwrap().name, "Rocket");

    assert_eq!(albums.get(1).unwrap().id, 2);
    assert_eq!(albums.get(1).unwrap().name, "Trick");
}

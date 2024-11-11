use include_sqlite_sql::{impl_sql, include_sql};
use momo::{artist::get_all_artists, GlobalState};
use rusqlite::Connection;
use std::sync::Mutex;

mod common;

include_sql!("sql/Tracks.sql");
include_sql!("sql/Albums.sql");
include_sql!("sql/Artists.sql");
include_sql!("sql/Playlists.sql");
include_sql!("sql/PlaylistTracks.sql");

#[test]
fn can_get_all_artists() {
    let state = GlobalState {
        connection: Mutex::new(Connection::open_in_memory().unwrap()),
    };

    common::create_tables(&state).unwrap();

    common::create_artist("Alex G", &state).unwrap();
    common::create_artist("Lizzy McAlpine", &state).unwrap();
    common::create_artist("The Daughters of Eve", &state).unwrap();

    let artists = get_all_artists(&state).unwrap();

    assert_eq!(artists.get(0).unwrap().name, "Alex G");

    assert_eq!(artists.get(1).unwrap().name, "Lizzy McAlpine");

    assert_eq!(artists.get(2).unwrap().name, "The Daughters of Eve");
}

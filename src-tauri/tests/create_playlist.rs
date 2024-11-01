use include_sqlite_sql::{impl_sql, include_sql};
use momo::{playlist::create_playlist, playlist::get_all_playlists, GlobalState};
use rusqlite::Connection;
use std::sync::Mutex;

mod common;

include_sql!("sql/Tracks.sql");
include_sql!("sql/Albums.sql");
include_sql!("sql/Artists.sql");
include_sql!("sql/Playlists.sql");
include_sql!("sql/PlaylistTracks.sql");

#[test]
fn can_create_playlist() {
    let state = GlobalState {
        connection: Mutex::new(Connection::open_in_memory().unwrap()),
    };

    common::create_tables(&state).unwrap();

    create_playlist("Playlist #1", &state).unwrap();
    create_playlist("Playlist #2", &state).unwrap();
    create_playlist("Playlist #3", &state).unwrap();

    let playlists = get_all_playlists(&state).unwrap();

    assert_eq!(playlists.get(0).unwrap().id, 1);
    assert_eq!(playlists.get(0).unwrap().name, "Playlist #1");

    assert_eq!(playlists.get(1).unwrap().id, 2);
    assert_eq!(playlists.get(1).unwrap().name, "Playlist #2");

    assert_eq!(playlists.get(2).unwrap().id, 3);
    assert_eq!(playlists.get(2).unwrap().name, "Playlist #3");
}

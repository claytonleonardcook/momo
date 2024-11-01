use include_sqlite_sql::{impl_sql, include_sql};
use momo::{album::get_all_albums, GlobalState};
use rusqlite::Connection;
use std::sync::Mutex;

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

    {
        let connection = state.connection.lock().unwrap();

        connection.create_artists_table().unwrap();
        connection.create_albums_table().unwrap();
        connection.create_tracks_table().unwrap();

        let aritst_id = connection
            .insert_artist("Alex G", |row| Ok(row.get_ref("id")?.as_i64()?))
            .unwrap();

        connection
            .insert_album("Rocket", aritst_id, |_row| Ok(()))
            .unwrap();

        connection
            .insert_album("Trick", aritst_id, |_row| Ok(()))
            .unwrap();
    }

    let albums = get_all_albums(&state).unwrap();

    assert_eq!(albums.get(0).unwrap().id, 1);
    assert_eq!(albums.get(0).unwrap().name, "Rocket");

    assert_eq!(albums.get(1).unwrap().id, 2);
    assert_eq!(albums.get(1).unwrap().name, "Trick");
}

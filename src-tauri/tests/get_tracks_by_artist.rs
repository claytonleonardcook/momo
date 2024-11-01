use include_sqlite_sql::{impl_sql, include_sql};
use momo::{track::get_tracks_by_artist, GlobalState};
use rusqlite::Connection;
use std::sync::Mutex;

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

    {
        let connection = state.connection.lock().unwrap();

        connection.create_artists_table().unwrap();
        connection.create_albums_table().unwrap();
        connection.create_tracks_table().unwrap();

        let artist_id = connection
            .insert_artist("Alex G", |row| Ok(row.get_ref("id")?.as_i64()?))
            .unwrap();

        {
            let album_id = connection
                .insert_album("Rocket", artist_id, |row| {
                    Ok(row.get_ref("id")?.as_i64()?)
                })
                .unwrap();

            connection
                .insert_track("Bobby", "./hello", album_id, |_row| Ok(()))
                .unwrap();

            connection
                .insert_track("Proud", "./world", album_id, |_row| Ok(()))
                .unwrap();
        }

        {
            let album_id = connection
                .insert_album("Trick", artist_id, |row| Ok(row.get_ref("id")?.as_i64()?))
                .unwrap();

            connection
                .insert_track("Memory", "./memory", album_id, |_row| Ok(()))
                .unwrap();
        }
    }

    let tracks = get_tracks_by_artist(1, &state).unwrap();

    assert_eq!(tracks.get(0).unwrap().id, 1);
    assert_eq!(tracks.get(0).unwrap().name, "Bobby");

    assert_eq!(tracks.get(1).unwrap().id, 2);
    assert_eq!(tracks.get(1).unwrap().name, "Proud");

    assert_eq!(tracks.get(2).unwrap().id, 3);
    assert_eq!(tracks.get(2).unwrap().name, "Memory");
}

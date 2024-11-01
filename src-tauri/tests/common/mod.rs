use include_sqlite_sql::{impl_sql, include_sql};
use momo::GlobalState;

include_sql!("sql/Tracks.sql");
include_sql!("sql/Albums.sql");
include_sql!("sql/Artists.sql");
include_sql!("sql/Playlists.sql");
include_sql!("sql/PlaylistTracks.sql");

pub fn create_tables(state: &GlobalState) -> Result<(), ()> {
    let connection = state.connection.lock().unwrap();

    connection.create_tracks_table().unwrap();
    connection.create_albums_table().unwrap();
    connection.create_artists_table().unwrap();
    connection.create_playlists_table().unwrap();
    connection.create_playlist_tracks_table().unwrap();

    Ok(())
}

// TODO: fix dead code warning
#[allow(dead_code)]
pub fn create_artist(name: &str, state: &GlobalState) -> Result<i64, ()> {
    let connection = state.connection.lock().unwrap();

    Ok(connection
        .insert_artist(name, |row| Ok(row.get_ref("id")?.as_i64()?))
        .unwrap())
}

// TODO: fix dead code warning
#[allow(dead_code)]
pub fn create_album(name: &str, artist_id: i64, state: &GlobalState) -> Result<i64, ()> {
    let connection = state.connection.lock().unwrap();

    Ok(connection
        .insert_album(name, artist_id, |row| Ok(row.get_ref("id")?.as_i64()?))
        .unwrap())
}

// TODO: fix dead code warning
#[allow(dead_code)]
pub fn create_track(name: &str, path: &str, album_id: i64, state: &GlobalState) -> Result<i64, ()> {
    let connection = state.connection.lock().unwrap();

    Ok(connection
        .insert_track(name, &path, album_id, |row| {
            Ok(row.get_ref("id")?.as_i64()?)
        })
        .unwrap())
}

// TODO: fix dead code warning
#[allow(dead_code)]
pub fn create_playlist(name: &str, state: &GlobalState) -> Result<i64, ()> {
    let connection = state.connection.lock().unwrap();

    Ok(connection
        .insert_playlist(name, |row| Ok(row.get_ref("id")?.as_i64()?))
        .unwrap())
}

-- name: create_tracks_table
-- Creates the tracks table
CREATE TABLE IF NOT EXISTS Tracks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    path TEXT NOT NULL,
    album_id INTEGER NOT NULL,
    FOREIGN KEY (album_id) REFERENCES Albums (id) ON DELETE CASCADE
)

-- name: insert_track->
-- Inserts a track into the tracks table
-- param: name: &str - name of track
-- param: path: &str - real path to track file
-- param: album_id: i64 - id of parent album
INSERT INTO Tracks (name, path, album_id)
    VALUES (:name, :path, :album_id)
    RETURNING id

-- name: get_track_path_by_id->
-- Get's a track's path by id from tracks table
-- param: track_id: i64 - the id of the track
SELECT id, path 
    FROM Tracks 
    WHERE id = :track_id

-- name: get_all_tracks?
-- Get's all tracks from tracks table
SELECT Tracks.id AS id, Tracks.name, Tracks.path, Tracks.album_id, Albums.name AS album_name
    FROM Tracks 
    JOIN Albums ON Tracks.album_id = Albums.id

-- name: get_tracks_by_artist?
-- Get all tracks for a specific artist
-- param: artist_name: &str - the name of the artist
SELECT Tracks.id AS id, Tracks.name, Tracks.path, Tracks.album_id, Albums.name AS album_name, Artists.name AS artist_name
    FROM Tracks
    JOIN Albums ON Tracks.album_id = Albums.id
    JOIN Artists ON Albums.artist_name = Artists.name
    WHERE Artists.name = :artist_name

-- name: get_tracks_by_album?
-- Get all tracks for a specific album
-- param: album_id: i64 - the id of the album
SELECT id, name, path, album_id
    FROM Tracks
    WHERE album_id = :album_id

-- name: delete_track
-- Deletes a track by id
-- param: track_id: i64 - the id of the track to delete
DELETE FROM Tracks
    WHERE id = :track_id

-- name: create_tracks_table
-- Creates the tracks table
CREATE TABLE IF NOT EXISTS Tracks (
    id INTEGER PRIMARY KEY,
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

-- name: get_all_tracks?
-- Get's all tracks from tracks table
SELECT id, name, path, album_id FROM Tracks

-- name: get_tracks_by_artist?
-- Get all tracks for a specific artist
-- param: artist_id: i64 - the id of the artist
SELECT Tracks.id, Tracks.name, Tracks.path, Tracks.album_id
    FROM Tracks
    JOIN Albums ON Tracks.album_id = Albums.id
    JOIN Artists ON Albums.artist_id = Artists.id
    WHERE Artists.id = :artist_id

-- name: get_tracks_by_album
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

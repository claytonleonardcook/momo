-- name: create_playlists_table
-- Creates the playlists table
CREATE TABLE IF NOT EXISTS Playlists (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
)

-- name: insert_playlist->
-- Inserts a playlist into the playlists table
-- param: name: &str - name of track
INSERT INTO Playlists (name)
    VALUES (:name)
    RETURNING id

-- name: get_all_playlists?
-- Get's all playlists from playlists table
SELECT id, name FROM Playlists

-- name: delete_playlist
-- Deletes a playlist by id
-- param: playlist_id: i64 - the id of the playlist to delete
DELETE FROM Playlists
    WHERE id = :playlist_id

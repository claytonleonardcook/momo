-- name: create_artists_table
-- Creates the artists table
CREATE TABLE IF NOT EXISTS Artists (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
)

-- name: insert_artist
-- Inserts a artist into the artists table
-- param: name: &str - name of artist
INSERT INTO Artists (name)
    VALUES (:name)

-- name: get_all_artists?
-- Gets all artists from artists table
SELECT id, name FROM Artists

-- name: delete_artist
-- Deletes an artist by id
-- param: artist_id: i64 - the id of the artist to delete
DELETE FROM Artists
    WHERE id = :artist_id;

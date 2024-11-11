-- name: create_artists_table
-- Creates the artists table
CREATE TABLE IF NOT EXISTS Artists (
    name TEXT NOT NULL PRIMARY KEY
)

-- name: insert_artist->
-- Inserts a artist into the artists table
-- param: name: &str - name of artist
INSERT OR IGNORE INTO Artists (name)
    VALUES (:name)
    RETURNING name

-- name: get_all_artists?
-- Gets all artists from artists table
SELECT name FROM Artists

-- name: delete_artist
-- Deletes an artist by id
-- param: artist_name: &str - the id of the artist to delete
DELETE FROM Artists
    WHERE name = :artist_name

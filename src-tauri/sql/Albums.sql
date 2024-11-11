-- name: create_albums_table
-- Creates the albums table
CREATE TABLE IF NOT EXISTS Albums (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    artist_name INTEGER NOT NULL,
    FOREIGN KEY (artist_name) REFERENCES Artists (name) ON DELETE CASCADE
)

-- name: insert_album->
-- Inserts a album into the albums table
-- param: name: &str - name of album
-- param: artist_name: &str - id of parent artist
INSERT OR IGNORE INTO Albums (name, artist_name)
    VALUES (:name, :artist_name)
    RETURNING id

-- name: get_all_albums?
-- Get's all albums from albums table
SELECT id, name, artist_name FROM Albums

-- name: get_albums_by_artist?
-- Get all albums for a specific artist
-- param: artist_name: &str - the id of the artist
SELECT id, name, artist_name
    FROM Albums
    WHERE artist_name = :artist_name

-- name: delete_album
-- Deletes an album by id
-- param: album_id: i64 - the id of the album to delete
DELETE FROM Albums
    WHERE id = :album_id

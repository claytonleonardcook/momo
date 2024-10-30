-- name: create_playlist_tracks_table
-- Creates the playlist_tracks table (junction table)
CREATE TABLE IF NOT EXISTS PlaylistTracks (
    playlist_id INTEGER NOT NULL,
    track_id INTEGER NOT NULL,
    FOREIGN KEY (playlist_id) REFERENCES Playlists (id) ON DELETE CASCADE,
    FOREIGN KEY (track_id) REFERENCES Tracks (id) ON DELETE CASCADE,
    PRIMARY KEY (playlist_id, track_id)
)

-- name: add_track_to_playlist
-- Adds a track to a playlist
-- param: playlist_id: i64 - the id of the playlist
-- param: track_id: i64 - the id of the track
INSERT INTO PlaylistTracks (playlist_id, track_id)
    VALUES (:playlist_id, :track_id)

-- name: get_tracks_in_playlist?
-- Get all tracks for a specific playlist
-- param: playlist_id: i64 - the id of the playlist
SELECT Tracks.id, Tracks.name, Tracks.path, Tracks.album_id
    FROM Tracks
    JOIN PlaylistTracks ON Tracks.id = PlaylistTracks.track_id
    WHERE PlaylistTracks.playlist_id = :playlist_id

-- name: delete_track_from_playlist
-- Deletes a track from a playlist
-- param: playlist_id: i64 - the id of the playlist
-- param: track_id: i64 - the id of the track
DELETE FROM PlaylistTracks
    WHERE playlist_id = :playlist_id AND track_id = :track_id

-- name: delete_all_tracks_from_playlist
-- Deletes all tracks from a playlist
-- param: playlist_id: i64 - the id of the playlist
DELETE FROM PlaylistTracks
    WHERE playlist_id = :playlist_id

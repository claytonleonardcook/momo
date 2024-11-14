export type Artist = {
  name: string;
};

export type Album = {
  id: string;
  name: string;
  artist_id: Artist["name"];
};

export type Track = {
  id: string;
  name: string;
  path: string;
  album_id: Album["id"];
};

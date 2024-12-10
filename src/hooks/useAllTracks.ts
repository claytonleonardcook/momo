import { Track } from "@/types/schema";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

function useAllTracks() {
  const [tracks, setTracks] = useState<Track[]>([]);

  useEffect(() => {
    invoke("get_all_tracks")
      .then((value) => setTracks(value as Track[]))
      .catch(console.error);
  }, []);

  return { tracks };
}

export default useAllTracks;

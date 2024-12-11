import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

type TrackPositionPayload = {
  position: number;
  total_duration: number;
};

function useTrackPosition() {
  const [position, setPosition] = useState<TrackPositionPayload>({
    total_duration: 0,
    position: 0,
  });

  useEffect(() => {
    const interval = setInterval(() => {
      invoke("get_track_position")
        .then((value) => setPosition(value as TrackPositionPayload))
        .catch(console.error);
    }, 500);

    return () => clearInterval(interval);
  }, []);

  return {
    totalDuration: position.total_duration,
    position: position.position,
  };
}

export default useTrackPosition;

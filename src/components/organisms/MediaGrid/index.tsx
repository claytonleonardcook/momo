import { HTMLAttributes, useEffect, useState } from "react";
import styles from "./style.module.scss";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { Track } from "@/types/schema";
import Card from "@/components/atoms/Card";
import { appCacheDir } from "@tauri-apps/api/path";

namespace MediaGrid {
  export type Props = HTMLAttributes<HTMLDivElement>;
}

const MediaGrid = ({ className, style, ...props }: MediaGrid.Props) => {
  const [cacheDirectory, setCacheDirectory] = useState<string>("");
  const [tracks, setTracks] = useState<Track[]>([]);

  useEffect(() => {
    invoke("get_all_tracks")
      .then((value) => setTracks(value as Track[]))
      .catch(console.error);

    appCacheDir().then(setCacheDirectory).catch(console.error);
  }, []);

  return (
    <section
      className={`${styles["media-grid"]} ${className ?? ""}`}
      {...props}
    >
      {tracks.map((track) => {
        return (
          <Card
            key={track.id}
            title={track.name}
            subtext={"hello"}
            image={convertFileSrc(
              `${cacheDirectory}/covers/${track.album_name}.jpeg`
            )}
            onPress={async () => {
              console.log("Play track");
            }}
          />
        );
      })}
    </section>
  );
};

export default MediaGrid;

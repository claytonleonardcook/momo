import { HTMLAttributes, useEffect, useState } from "react";
import styles from "./style.module.scss";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import Card from "@/components/atoms/Card";
import { appCacheDir } from "@tauri-apps/api/path";
import { useAllTracks } from "@/hooks";

namespace MediaGrid {
  export type Props = HTMLAttributes<HTMLDivElement>;
}

const MediaGrid = ({ className, style, ...props }: MediaGrid.Props) => {
  const [cacheDirectory, setCacheDirectory] = useState<string>("");
  const { tracks } = useAllTracks();

  useEffect(() => {
    appCacheDir().then(setCacheDirectory).catch(console.error);
  }, []);

  function onCardPress(trackId: number) {
    invoke("play_track", {
      trackId,
    }).catch(console.error);
  }

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
            subtext={track.album_name}
            image={convertFileSrc(
              `${cacheDirectory}/covers/${track.album_name}.jpeg`
            )}
            onPress={() => {
              onCardPress(track.id);
            }}
          />
        );
      })}
    </section>
  );
};

export default MediaGrid;

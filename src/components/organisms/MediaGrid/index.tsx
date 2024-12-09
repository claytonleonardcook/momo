import { HTMLAttributes, useEffect, useState } from "react";
import styles from "./style.module.scss";
import { invoke } from "@tauri-apps/api/core";
import { Track } from "@/types/schema";
import Card from "@/components/atoms/Card";

namespace MediaGrid {
  export type Props = HTMLAttributes<HTMLDivElement>;
}

const MediaGrid = ({ className, style, ...props }: MediaGrid.Props) => {
  const [tracks, setTracks] = useState<Track[]>([]);

  useEffect(() => {
    invoke("get_all_tracks")
      .then((value) => setTracks(value as Track[]))
      .catch(console.error);
  }, []);

  return (
    <section
      className={`${styles["media-grid"]} ${className ?? ""}`}
      {...props}
    >
      {tracks.map((track) => {
        return (
          <Card
            title={track.name}
            subtext={"hello"}
            image={"./ninjatuna.jpg"}
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

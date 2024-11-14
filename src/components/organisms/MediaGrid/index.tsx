import { HTMLAttributes, useContext, useEffect, useState } from "react";
import styles from "./style.module.scss";
import { invoke } from "@tauri-apps/api/core";
import { Track } from "@/types/schema";
import Card from "@/components/atoms/Card";
import Button from "@/components/atoms/Button";
import { open } from "@tauri-apps/plugin-dialog";
import { AudioPlayerContext } from "@/contexts";

namespace MediaGrid {
  export type Props = HTMLAttributes<HTMLDivElement>;
}

const MediaGrid = ({ className, style, ...props }: MediaGrid.Props) => {
  const audioPlayerContext = useContext(AudioPlayerContext);
  const [tracks, setTracks] = useState<Track[]>([]);

  useEffect(() => {
    invoke("get_all_tracks")
      .then((value) => setTracks(value as Track[]))
      .catch(console.error);
  }, []);

  async function onSelectMusicFolderPress() {
    const directory = await open({
      multiple: false,
      directory: true,
    });

    await invoke("set_library_directory", {
      directory,
    });

    const tracks = (await invoke("get_all_tracks")) as Track[];

    setTracks(tracks);
  }

  return (
    <section
      className={`${styles["media-grid"]} ${className ?? ""}`}
      {...props}
    >
      {tracks.map((track) => {
        return (
          <div>
            <Card
              title={track.name}
              subtext={"hello"}
              image={"./ninjatuna.jpg"}
              onPress={async () => {
                console.log("Play track");
              }}
            />
          </div>
        );
      })}
      {tracks && (
        <Button onPress={onSelectMusicFolderPress}>
          Select a Music Folder
        </Button>
      )}
    </section>
  );
};

export default MediaGrid;

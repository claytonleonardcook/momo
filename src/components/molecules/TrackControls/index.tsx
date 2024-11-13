import { HTMLAttributes, useState } from "react";
import styles from "./style.module.scss";
import Button from "@/components/atoms/Button";
import Slider from "@/components/atoms/Slider";
import Icon from "@/components/atoms/Icon";
import {
  FaBackward,
  FaBars,
  FaForward,
  FaPause,
  FaPlay,
  FaVolumeMute,
} from "react-icons/fa";
import { FaRepeat, FaShuffle, FaVolumeHigh } from "react-icons/fa6";
import { PressEvent } from "react-aria-components";
import { invoke } from "@tauri-apps/api";

type TrackControlsProps = HTMLAttributes<HTMLDivElement> & {
  trackDuration: number;
};

const TrackControls = ({
  className,
  trackDuration,
  style,
  ...props
}: TrackControlsProps) => {
  const [isPlaying, setIsPlaying] = useState<boolean>(false);
  const [volume, setVolume] = useState(0);

  function onVolumeChange(value: number | number[]) {
    if (typeof value === "number") {
      setVolume(value);
    }
  }

  function onVolumeChangeEnd(value: number | number[]) {
    if (typeof value === "number") {
      console.log(`Change volume to ${volume}%`);
    }
  }

  function onBackwardPress(event: PressEvent) {
    console.log("Backward press");
  }

  function onPlayPausePress(event: PressEvent) {
    if (isPlaying) {
      invoke("pause").catch(console.error);
    } else {
      invoke("play").catch(console.error);
    }

    setIsPlaying((state) => !state);
  }

  function onForwardPress(event: PressEvent) {
    console.log("Forward press");
  }

  function onShufflePress(event: PressEvent) {
    console.log("Shuffle press");
  }

  function onLoopPress(event: PressEvent) {
    console.log("Loop press");
  }

  function onMenuPress(event: PressEvent) {
    console.log("Menu press");
  }

  return (
    <section
      className={`${styles["track-controls"]} ${className ?? ""}`}
      {...props}
    >
      <div className={styles["track-controls--controls"]}>
        <Button
          onPress={onBackwardPress}
          className={styles["track-controls--backward"]}
          variant={"text"}
        >
          <Icon icon={FaBackward} />
        </Button>
        <Button
          onPress={onPlayPausePress}
          className={styles["track-controls--pause"]}
          variant={"text"}
        >
          <Icon icon={isPlaying ? FaPause : FaPlay} />
        </Button>
        <Button
          onPress={onForwardPress}
          className={styles["track-controls--forward"]}
          variant={"text"}
        >
          <Icon icon={FaForward} />
        </Button>
      </div>
      <div className={styles["track-controls--time"]}>
        <span>00:00</span>
        <Slider
          className={styles["track-controls--time__track"]}
          color={"blue"}
          defaultValue={100}
        >
          <Slider.Track />
        </Slider>
        <span>3:46</span>
      </div>
      <div className={styles["track-controls--volume"]}>
        <Icon icon={volume === 0 ? FaVolumeMute : FaVolumeHigh} size={32} />
        <Slider
          className={styles["track-controls--volume__track"]}
          color={"blue"}
          value={volume}
          onChangeEnd={onVolumeChangeEnd}
          onChange={onVolumeChange}
        >
          <Slider.Track />
        </Slider>
      </div>
      <Button
        onPress={onShufflePress}
        className={styles["track-controls--shuffle"]}
        variant={"text"}
      >
        <Icon icon={FaShuffle} />
      </Button>
      <Button
        onPress={onLoopPress}
        className={styles["track-controls--loop"]}
        variant={"text"}
      >
        <Icon icon={FaRepeat} />
      </Button>
      <Button
        onPress={onMenuPress}
        className={styles["track-controls--menu"]}
        variant={"text"}
      >
        <Icon icon={FaBars} />
      </Button>
    </section>
  );
};

export default TrackControls;
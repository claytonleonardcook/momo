import { HTMLAttributes, useState } from "react";
import styles from "./style.module.scss";
import Button from "@/components/atoms/Button";
import Slider from "@/components/atoms/Slider";
import Icon from "@/components/atoms/Icon";
import { FaBackward, FaBars, FaForward, FaPause, FaPlay } from "react-icons/fa";
import { FaRepeat, FaShuffle } from "react-icons/fa6";
import { invoke } from "@tauri-apps/api/core";
import VolumeSlider from "@/components/molecules/VolumeSlider";
import useSettingsDialog from "@/hooks/useSettingsDialog";

type TrackControlsProps = HTMLAttributes<HTMLDivElement> & {
  trackDuration: number;
};

const TrackControls = ({
  className,
  trackDuration,
  style,
  ...props
}: TrackControlsProps) => {
  const settingsDialogRef = useSettingsDialog();
  const [isPlaying, setIsPlaying] = useState<boolean>(false);

  function onBackwardPress() {
    console.log("Backward press");
  }

  function onPlayPausePress() {
    if (isPlaying) {
      invoke("pause").catch(console.error);
    } else {
      invoke("play").catch(console.error);
    }

    setIsPlaying((state) => !state);
  }

  function onForwardPress() {
    console.log("Forward press");
  }

  function onShufflePress() {
    console.log("Shuffle press");
  }

  function onLoopPress() {
    console.log("Loop press");
  }

  function onMenuPress() {
    settingsDialogRef.current?.showModal();
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
      <VolumeSlider>
        <VolumeSlider.Icon />
        <VolumeSlider.Slider />
      </VolumeSlider>
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

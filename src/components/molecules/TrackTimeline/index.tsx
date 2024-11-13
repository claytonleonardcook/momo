import { HTMLAttributes, useEffect, useState } from "react";
import styles from "./style.module.scss";
import Slider from "@/components/atoms/Slider";
import { invoke } from "@tauri-apps/api";

namespace TrackTimeline {
  export type Props = HTMLAttributes<HTMLDivElement>;
}

const TrackTimeline = ({
  className,
  children,
  ...props
}: TrackTimeline.Props) => {
  const [time, setTime] = useState<number>(0);

  // useEffect(() => {
  //   setInterval(() => setTime((value) => ++value), 1000);
  // }, []);

  function onTimelineChange(value: number | number[]) {
    if (typeof value === "number") {
      // setVolume(value);
    }
  }

  function onTimelineChangeEnd(value: number | number[]) {
    if (typeof value === "number") {
      // invoke("set_volume", {
      //   volume,
      // }).catch(console.error);
    }
  }

  return (
    <div className={styles["track-timeline"]} {...props}>
      <span>00:00</span>
      <Slider
        className={styles["track-timeline__track"]}
        color={"blue"}
        value={time}
      >
        <Slider.Track />
      </Slider>
      <span>3:46</span>
    </div>
  );
};

export default TrackTimeline;

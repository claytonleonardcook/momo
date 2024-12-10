import { HTMLAttributes, useState } from "react";
import styles from "./style.module.scss";
import Slider from "@/components/atoms/Slider";

namespace TrackTimeline {
  export type Props = HTMLAttributes<HTMLDivElement>;
}

const TrackTimeline = ({
  className,
  children,
  ...props
}: TrackTimeline.Props) => {
  const [time] = useState<number>(0);

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

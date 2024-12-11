import { HTMLAttributes } from "react";
import styles from "./style.module.scss";
import Slider from "@/components/atoms/Slider";
import useTrackPosition from "@/hooks/useTrackPosition";
import { trackTimeFormat } from "@/utils";

namespace TrackTimeline {
  export type Props = HTMLAttributes<HTMLDivElement>;
}

const TrackTimeline = ({
  className = "",
  children,
  ...props
}: TrackTimeline.Props) => {
  const { position, totalDuration } = useTrackPosition();

  return (
    <div className={`${styles["track-timeline"]} ${className}`} {...props}>
      <span>{trackTimeFormat(position)}</span>
      <Slider
        className={styles["track-timeline__track"]}
        color={"blue"}
        value={position}
        minValue={0}
        maxValue={totalDuration}
      >
        <Slider.Track />
      </Slider>
      <span>{trackTimeFormat(totalDuration)}</span>
    </div>
  );
};

export default TrackTimeline;

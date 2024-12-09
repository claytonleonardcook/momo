import { createContext, HTMLAttributes, useContext, useState } from "react";
import styles from "./style.module.scss";
import Slider from "@/components/atoms/Slider";
import Icon from "@/components/atoms/Icon";
import { FaVolumeMute } from "react-icons/fa";
import { FaVolumeHigh } from "react-icons/fa6";
import { invoke } from "@tauri-apps/api/core";
import { SliderProps } from "react-aria-components";

namespace VolumeSlider {
  export type Props = HTMLAttributes<HTMLDivElement>;
}

const VolumeContext = createContext<
  [number, SliderProps["onChange"], SliderProps["onChangeEnd"]]
>([0, () => {}, () => {}]);

const VolumeSlider = ({
  className = "",
  children,
  ...props
}: VolumeSlider.Props) => {
  const [volume, setVolume] = useState<number>(1);

  function onVolumeChange(value: number | number[]) {
    if (typeof value === "number") {
      setVolume(value);
    }
  }

  function onVolumeChangeEnd(value: number | number[]) {
    if (typeof value === "number") {
      // invoke("set_volume", {
      //   volume,
      // }).catch(console.error);
    }
  }

  return (
    <VolumeContext.Provider value={[volume, onVolumeChange, onVolumeChangeEnd]}>
      <div className={`${styles["volume-slider"]} ${className}`} {...props}>
        {children}
      </div>
    </VolumeContext.Provider>
  );
};

VolumeSlider.Icon = () => {
  const [volume] = useContext(VolumeContext);

  return <Icon icon={volume === 0 ? FaVolumeMute : FaVolumeHigh} size={32} />;
};

VolumeSlider.Slider = () => {
  const [volume, onVolumeChange, onVolumeChangeEnd] = useContext(VolumeContext);

  return (
    <Slider
      className={styles["volume-slider__track"]}
      color={"blue"}
      value={volume}
      minValue={0}
      maxValue={2}
      step={0.01}
      onChangeEnd={onVolumeChangeEnd}
      onChange={onVolumeChange}
    >
      <Slider.Track />
    </Slider>
  );
};

export default VolumeSlider;

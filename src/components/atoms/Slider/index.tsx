import { ComponentProps, CSSProperties } from "react";
import {
  Slider as AriaSlider,
  SliderTrack as AriaSliderTrack,
  SliderThumb as AriaSliderThumb,
} from "react-aria-components";
import { TypeColors } from "@/types/colors";
import styles from "./style.module.scss";

type SliderProps = ComponentProps<typeof AriaSlider> & {
  color?: TypeColors;
};

interface SliderCSSProperties extends CSSProperties {
  "--slider-color": `var(--${SliderProps["color"]})`;
  "--slider-color-drag": `var(--${SliderProps["color"]}--light-25)`;
}

const Slider = ({
  className = "",
  color = "blue",
  children,
  orientation = "horizontal",
  style,
  ...props
}: SliderProps) => {
  return (
    <AriaSlider
      className={`${styles["slider"]} ${className}`}
      orientation={orientation}
      style={
        {
          "--slider-color": `var(--${color})`,
          "--slider-color-drag": `var(--${color}--light-25)`,
          ...style,
        } as SliderCSSProperties
      }
      {...props}
    >
      {children}
    </AriaSlider>
  );
};

type SliderTrackProps = ComponentProps<typeof AriaSliderTrack>;

Slider.Track = ({ ...props }: SliderTrackProps) => {
  return (
    <AriaSliderTrack className={styles["slider__track"]} {...props}>
      {({ state }) => (
        <>
          <div
            className={styles["slider__track__track"]}
            style={{ width: state.getThumbPercent(0) * 100 + "%" }}
          />
          <AriaSliderThumb className={styles["slider__thumb"]} />
        </>
      )}
    </AriaSliderTrack>
  );
};

export default Slider;

import { TypeColors } from "@/types/colors";
import React from "react";
import { IconType } from "react-icons";

type Props = {
  /**
   * Icon component from react-icons.
   * @see https://react-icons.github.io/react-icons/
   * */
  icon: IconType;
  /**
   * Fill of the icon.
   */
  fill?: TypeColors | "currentColor";
  /**
   * Stroke of the icon.
   */
  stroke?: TypeColors;
  /**
   * Size of the icon in pixels, stick to multiples of 8.
   */
  size?: number;
};

const Icon = ({
  icon,
  fill = "currentColor",
  stroke = "none",
  size = 16,
}: Props) => {
  return React.createElement(icon, {
    className: `fill-${fill} stroke-${stroke} stroke-${stroke}--light-50`,
    style: { width: size, height: size },
  });
};

export default Icon;

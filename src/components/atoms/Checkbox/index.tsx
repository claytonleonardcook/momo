import { ComponentProps, CSSProperties } from "react";
import { Checkbox as AriaCheckbox } from "react-aria-components";
import { TypeColors } from "@/types/colors";
import styles from "./style.module.scss";
import Icon from "../Icon";
import { IconType } from "react-icons";
import { FaCheck } from "react-icons/fa";

type CheckboxProps = ComponentProps<typeof AriaCheckbox> & {
  color?: TypeColors;
};

interface CheckboxCSSProperties extends CSSProperties {
  "--checkbox-color": `var(--${CheckboxProps["color"]})`;
}

const Checkbox = ({
  className = "",
  color = "blue",
  children,
  style,
  ...props
}: CheckboxProps) => {
  return (
    <AriaCheckbox
      className={`${styles["checkbox"]} ${className}`}
      style={
        {
          "--checkbox-color": `var(--${color})`,
          ...style,
        } as CheckboxCSSProperties
      }
      {...props}
    >
      {children}
    </AriaCheckbox>
  );
};

type CheckboxInputProps = {
  icon?: IconType;
};

Checkbox.Input = ({ icon = FaCheck }: CheckboxInputProps) => {
  return (
    <div className={`${styles["checkbox__input"]}`}>
      <Icon icon={icon} />
    </div>
  );
};

export default Checkbox;

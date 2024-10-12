import { ComponentProps, CSSProperties } from "react";
import {
  RadioGroup as AriaRadioGroup,
  Radio as AriaRadio,
} from "react-aria-components";
import { TypeColors } from "@/types/colors";
import styles from "./style.module.scss";

type RadioGroupProps = ComponentProps<typeof AriaRadioGroup> & {
  color?: TypeColors;
};

interface RadioGroupCSSProperties extends CSSProperties {
  "--radio-group-color": `var(--${RadioGroupProps["color"]})`;
}

const RadioGroup = ({
  className = "",
  color = "blue",
  children,
  style,
  ...props
}: RadioGroupProps) => {
  return (
    <AriaRadioGroup
      className={`${styles["radio-group"]} ${className}`}
      style={
        {
          "--radio-group-color": `var(--${color})`,
          ...style,
        } as RadioGroupCSSProperties
      }
      {...props}
    >
      {children}
    </AriaRadioGroup>
  );
};

type RadioButtonProps = ComponentProps<typeof AriaRadio>;

RadioGroup.RadioButton = ({
  className = "",
  children,
  ...props
}: RadioButtonProps) => {
  return (
    <AriaRadio
      className={`${styles["radio-group__radio-button"]} ${className}`}
      {...props}
    >
      {children}
    </AriaRadio>
  );
};

export default RadioGroup;

import { ComponentProps, CSSProperties } from "react";
import { Button as AriaButton } from "react-aria-components";
import { TypeColors } from "@/types/colors";
import styles from "./style.module.scss";

type ButtonProps = ComponentProps<typeof AriaButton> & {
  variant?: "standard" | "outlined" | "text";
  color?: TypeColors;
};

interface ButtonCSSProperties extends CSSProperties {
  "--button-color": `var(--${ButtonProps["color"]})`;
  "--button-hover-color": `var(--${ButtonProps["color"]}--light-50)`;
}

const Button = ({
  className,
  variant = "standard",
  color = "blue",
  children,
  style,
  ...props
}: ButtonProps) => {
  return (
    <AriaButton
      className={`${styles["button"]} ${styles[`button--${variant}`]} ${className ?? ""}`}
      style={
        {
          "--button-color": `var(--${color})`,
          "--button-hover-color": `var(--${color}--light-50)`,
          ...style,
        } as ButtonCSSProperties
      }
      {...props}
    >
      {children}
    </AriaButton>
  );
};

export default Button;

import {
  ComponentProps,
  createContext,
  CSSProperties,
  ReactElement,
  useContext,
} from "react";
import {
  Select as AriaSelect,
  SelectValue as AriaSelectValue,
  Label as AriaLabel,
  Popover as AriaPopover,
  ListBox as AriaListBox,
  ListBoxItem as AriaListBoxItem,
  Button as AriaButton,
} from "react-aria-components";
import { TypeColors } from "@/types/colors";
import { IconType } from "react-icons";
import Icon from "../Icon";
import { FaUpDown } from "react-icons/fa6";
import styles from "./style.module.scss";

const DropdownColorContext = createContext("");

type DropdownProps = ComponentProps<typeof AriaSelect> & {
  color?: TypeColors;
};

interface DropdownCSSProperties extends CSSProperties {
  "--dropdown-color": `var(--${DropdownProps["color"]})`;
}

const Dropdown = ({
  className = "",
  color = "blue",
  children,
  style,
  ...props
}: DropdownProps): ReactElement => {
  return (
    <DropdownColorContext.Provider value={color}>
      <AriaSelect
        className={`${styles["dropdown"]} ${className}`}
        style={
          {
            "--dropdown-color": `var(--${color})`,
            ...style,
          } as DropdownCSSProperties
        }
        {...props}
      >
        {children}
      </AriaSelect>
    </DropdownColorContext.Provider>
  );
};

type DropdownLabelProps = ComponentProps<typeof AriaLabel>;

Dropdown.Label = ({
  children,
  className = "",
  ...props
}: DropdownLabelProps): ReactElement => {
  return (
    <AriaLabel
      className={`${styles["dropdown__label"]} ${className}`}
      {...props}
    >
      {children}
    </AriaLabel>
  );
};

type DropdownButtonProps = ComponentProps<typeof AriaButton> & {
  icon?: IconType;
};

Dropdown.Button = ({
  children,
  className = "",
  icon = FaUpDown,
  ...props
}: DropdownButtonProps): ReactElement => {
  return (
    <AriaButton
      className={`${styles["dropdown__button"]} ${className}`}
      {...props}
    >
      <AriaSelectValue />
      <span aria-hidden={"true"}>
        <Icon icon={icon} />
      </span>
    </AriaButton>
  );
};

type DropdownOptionsProps = ComponentProps<typeof AriaPopover> & {
  options: string[];
};

Dropdown.Options = ({
  options,
  className = "",
  style,
  ...props
}: DropdownOptionsProps): ReactElement => {
  const color = useContext(DropdownColorContext);

  return (
    <AriaPopover
      className={`${className}`}
      style={
        {
          "--dropdown-color": `var(--${color})`,
          ...style,
        } as DropdownCSSProperties
      }
      {...props}
    >
      <AriaListBox className={styles["dropdown__options"]}>
        {options.map((option) => (
          <AriaListBoxItem
            id={option.toLowerCase()}
            className={styles["dropdown__options__option"]}
          >
            {option}
          </AriaListBoxItem>
        ))}
      </AriaListBox>
    </AriaPopover>
  );
};

export default Dropdown;

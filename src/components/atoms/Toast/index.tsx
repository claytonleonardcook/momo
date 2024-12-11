import { useToast } from "@/hooks";
import { CSSProperties } from "react";
import style from "./style.module.scss";
import { TypeColors } from "@/types/colors";

export const ToastContainer = () => {
  const { toasts } = useToast();

  return (
    <div className={style["toast-container"]}>
      {toasts.map((toast) => (
        <Toast {...toast} />
      ))}
    </div>
  );
};

type ToastProps = {
  message: string;
  color?: TypeColors;
};

interface ToastCSSProperties extends CSSProperties {
  "--toast-color": `var(--${ToastProps["color"]})`;
}

const Toast = ({ message, color = "blue" }: ToastProps) => {
  return (
    <div
      className={style.toast}
      style={
        {
          "--toast-color": `var(--${color})`,
        } as ToastCSSProperties
      }
    >
      {message}
    </div>
  );
};

export default Toast;

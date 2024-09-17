import ToastContext from "@/contexts/ToastContext";
import { useToast } from "@/hooks/useToast";
import { TypeToast } from "@/types/toasts";
import { CSSProperties, ReactNode, useCallback, useState } from "react";
import style from "./style.module.scss";
import { TypeColors } from "@/types/colors";

export const ToastProvider: React.FC<{ children: ReactNode }> = ({
  children,
}) => {
  const [toasts, setToasts] = useState<TypeToast[]>([]);

  const addToast = useCallback(
    (
      message: string,
      color: TypeColors = "base",
      options: Partial<TypeToast> = {}
    ) => {
      const id = Date.now();
      setToasts((prevToasts) => [
        ...prevToasts,
        { id, message, color, ...options },
      ]);

      setTimeout(() => {
        setToasts((prevToasts) =>
          prevToasts.filter((toast) => toast.id !== id)
        );
      }, options.duration || 3000);
    },
    []
  );

  return (
    <ToastContext.Provider value={{ toasts, addToast }}>
      {children}
    </ToastContext.Provider>
  );
};

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

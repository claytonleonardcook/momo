import { TypeColors } from "@/types/colors";
import { TypeToast } from "@/types/toasts";
import { ReactNode, useCallback, useState } from "react";
import ToastContext from "./ToastContext";

export namespace ToastProvider {
  export type Props = {
    children: ReactNode;
  };
}

function ToastProvider({ children }: ToastProvider.Props) {
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
}

export default ToastProvider;

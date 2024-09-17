import { TypeColors } from "@/types/colors";
import { TypeToast } from "@/types/toasts";
import { createContext } from "react";

type TypeToastContext = {
  toasts: TypeToast[];
  addToast: (
    message: string,
    color?: TypeColors,
    options?: Partial<TypeToast>
  ) => void;
};

const ToastContext = createContext<TypeToastContext | undefined>(undefined);

export default ToastContext;

import { TypeColors } from "./colors";

export type TypeToast = {
  id: number;
  message: string;
  color?: TypeColors;
  duration?: number;
};

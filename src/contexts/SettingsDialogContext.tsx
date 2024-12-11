import { createContext, MutableRefObject } from "react";

type TypeSettingsDialogContext = MutableRefObject<HTMLDialogElement | null>;

const SettingsDialogContext = createContext<
  TypeSettingsDialogContext | undefined
>(undefined);

export default SettingsDialogContext;

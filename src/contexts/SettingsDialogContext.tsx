import { createContext, MutableRefObject, ReactNode, useRef } from "react";

type TypeSettingsDialogContext = MutableRefObject<HTMLDialogElement | null>;

const SettingsDialogContext = createContext<
  TypeSettingsDialogContext | undefined
>(undefined);

export namespace SettingsDialogProvider {
  export type Props = {
    children: ReactNode;
  };
}

export const SettingsDialogProvider = (props: SettingsDialogProvider.Props) => {
  const ref = useRef<HTMLDialogElement | null>(null);

  return (
    <SettingsDialogContext.Provider value={ref}>
      {props.children}
    </SettingsDialogContext.Provider>
  );
};

export default SettingsDialogContext;

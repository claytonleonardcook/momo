import { ReactNode, useRef } from "react";
import SettingsDialogContext from "./SettingsDialogContext";

export namespace SettingsDialogProvider {
  export type Props = {
    children: ReactNode;
  };
}

function SettingsDialogProvider(props: SettingsDialogProvider.Props) {
  const ref = useRef<HTMLDialogElement | null>(null);

  return (
    <SettingsDialogContext.Provider value={ref}>
      {props.children}
    </SettingsDialogContext.Provider>
  );
}

export default SettingsDialogProvider;

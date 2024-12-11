import { ReactNode } from "react";
import { SettingsDialogProvider, ThemeProvider, ToastProvider } from ".";

export namespace Providers {
  export type Props = {
    children: ReactNode;
  };
}

function Providers({ children }: Providers.Props) {
  return (
    <ThemeProvider>
      <SettingsDialogProvider>
        <ToastProvider>{children}</ToastProvider>
      </SettingsDialogProvider>
    </ThemeProvider>
  );
}

export default Providers;

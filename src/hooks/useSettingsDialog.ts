import { SettingsDialogContext } from "@/contexts";
import { useContext } from "react";

const useSettingsDialog = () => {
  const context = useContext(SettingsDialogContext);

  if (!context) {
    throw new Error(
      "useSettingsDialog must be used within a SettingsDialogProvider"
    );
  }

  return context;
};

export default useSettingsDialog;

import Dashboard from "@/pages/Dashboard";
import { useContext, useEffect } from "react";
import ThemeContext from "@/contexts/ThemeContext";
import { ToastContainer, ToastProvider } from "@/components/atoms/Toast";
import { SettingsDialogProvider } from "./contexts/SettingsDialogContext";
import SettingsDialog from "./components/organisms/SettingsDialog";

const App = () => {
  const themeContext = useContext(ThemeContext);

  useEffect(() => {
    console.log(themeContext);
    document.body.setAttribute("data-theme", themeContext ?? "light");
  }, [themeContext]);

  return (
    <SettingsDialogProvider>
      <ToastProvider>
        <Dashboard />
        <ToastContainer />
        <SettingsDialog />
      </ToastProvider>
    </SettingsDialogProvider>
  );
};

export default App;

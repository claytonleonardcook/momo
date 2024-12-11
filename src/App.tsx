import Dashboard from "@/pages/Dashboard";
import { ToastContainer } from "@/components/atoms/Toast";
import SettingsDialog from "@/components/organisms/SettingsDialog";
import { Providers } from "@/contexts";

const App = () => {
  return (
    <Providers>
      <Dashboard />
      <ToastContainer />
      <SettingsDialog />
    </Providers>
  );
};

export default App;

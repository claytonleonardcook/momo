import Dashboard from "@/pages/Dashboard";
import { useContext, useEffect } from "react";
import ThemeContext from "@/contexts/ThemeContext";
import { ToastContainer, ToastProvider } from "@/components/atoms/Toast";

const App = () => {
  const themeContext = useContext(ThemeContext);

  useEffect(() => {
    console.log(themeContext);
    document.body.setAttribute("data-theme", themeContext ?? "light");
  }, [themeContext]);

  return (
    <ToastProvider>
      <Dashboard />
      <ToastContainer />
    </ToastProvider>
  );
};

export default App;

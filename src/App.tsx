import Dashboard from "@/pages/Dashboard";
import { useContext, useEffect } from "react";
import ThemeContext from "./contexts/ThemeContext";

const App = () => {
  const themeContext = useContext(ThemeContext);

  useEffect(() => {
    console.log(themeContext);
    document.body.setAttribute("data-theme", themeContext ?? "light");
  }, [themeContext]);

  return <Dashboard />;
};

export default App;

import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "@/styles/global.scss";
import ThemeContext from "@/contexts/ThemeContext";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ThemeContext.Provider value={"dark"}>
      <App />
    </ThemeContext.Provider>
  </React.StrictMode>
);

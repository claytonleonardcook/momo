import { ReactNode, useCallback, useEffect, useState } from "react";
import ThemeContext, { TypeThemeContext } from "./ThemeContext";

export namespace ThemeProvider {
  export type Props = {
    children: ReactNode;
  };
}

function ThemeProvider({ children }: ThemeProvider.Props) {
  const [theme, setTheme] = useState<TypeThemeContext["theme"]>("dark");

  useEffect(() => {
    document.body.setAttribute("data-theme", theme ?? "dark");
  }, [theme]);

  const setLight = useCallback(() => setTheme("light"), []);

  const setDark = useCallback(() => setTheme("dark"), []);

  return (
    <ThemeContext.Provider value={{ theme: theme, setLight, setDark }}>
      {children}
    </ThemeContext.Provider>
  );
}

export default ThemeProvider;

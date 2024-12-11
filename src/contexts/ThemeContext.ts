import { createContext } from "react";

export type TypeThemeContext = {
  theme: "light" | "dark" | null;
  setLight: () => void;
  setDark: () => void;
};

const ThemeContext = createContext<TypeThemeContext>({
  theme: "dark",
  setLight: () => {},
  setDark: () => {},
});

export default ThemeContext;

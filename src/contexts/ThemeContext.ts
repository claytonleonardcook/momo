import { createContext } from "react";

const ThemeContext = createContext<"light" | "dark" | null>(null);

export default ThemeContext;

import { addons } from "@storybook/manager-api";
import { CatppuccinMocha } from "./Catppuccin";

export const THEME = CatppuccinMocha;

addons.setConfig({
  theme: THEME,
});

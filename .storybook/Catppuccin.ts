import { create } from "@storybook/theming/create";

export const CatppuccinLatte = create({
  base: "light",
  brandTitle: "Momo",
  brandImage: "/logo.svg",
  brandTarget: "_self",

  // Typography
  fontBase: '"Open Sans", sans-serif',
  fontCode: "monospace",

  // Text colors
  textColor: "#10162F",
  textInverseColor: "#ffffff",

  // Toolbar default and active colors
  barTextColor: "#9E9E9E",
  barSelectedColor: "#585C6D",
  barHoverColor: "#585C6D",
  barBg: "#ffffff",

  // Form colors
  inputBg: "#ffffff",
  inputBorder: "#10162F",
  inputTextColor: "#10162F",
  inputBorderRadius: 2,
});

export const CatppuccinMocha = create({
  base: "dark",
  brandTitle: "Momo",
  brandImage: "/logo.svg",
  brandTarget: "_blank",

  // Typography
  fontBase: '"Open Sans", sans-serif',
  fontCode: "monospace",

  // Text colors
  textColor: "#cdd6f4",
  textInverseColor: "#ffffff",

  // Toolbar default and active colors
  barTextColor: "#9E9E9E",
  barSelectedColor: "#585C6D",
  barHoverColor: "#585C6D",
  barBg: "#11111b",

  // Form colors
  inputBg: "#11111b",
  inputBorder: "#10162F",
  inputTextColor: "#cdd6f4",
  inputBorderRadius: 2,

  appBg: "#11111b",
  appPreviewBg: "#313244",
  appContentBg: "#11111b",
});

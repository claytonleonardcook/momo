import React from "react";
import type { Preview } from "@storybook/react";
import ThemeContext from "../src/contexts/ThemeContext";
import "./global.scss";
import { THEME } from "./manager";

const preview: Preview = {
  tags: ["autodocs"],
  globalTypes: {
    theme: {
      description: "Global theme for components",
      defaultValue: "light",
      toolbar: {
        title: "Theme",
        icon: "paintbrush",
        items: [
          { value: "light", right: "🌞", title: "Light" },
          { value: "dark", right: "🌚", title: "Dark" },
        ],
        dynamicTitle: true,
      },
    },
  },
  decorators: [
    (Story, context) => {
      document.body.setAttribute("data-theme", context.globals.theme);

      return (
        <ThemeContext.Provider value={context.globals.theme}>
          <Story />
        </ThemeContext.Provider>
      );
    },
  ],
  parameters: {
    controls: {
      exclude: /(children)/g,
    },
    options: {
      storySort: {
        order: [
          "General",
          "Components",
          "Atoms",
          "Molecules",
          "Organisms",
          "Pages",
          "Styles",
        ],
      },
    },
    docs: {
      theme: THEME,
      toc: {
        contentsSelector: ".sbdocs-content",
        headingSelector: "h1, h2, h3",
        ignoreSelector: "#primary",
        title: "Table of Contents",
        disable: false,
        unsafeTocbotOptions: {
          orderedList: false,
        },
      },
    },
    backgrounds: {
      default: "White",
      values: [
        { name: "Rosewater", value: "var(--rosewater)" },
        { name: "Flamingo", value: "var(--flamingo)" },
        { name: "Pink", value: "var(--pink)" },
        { name: "Mauve", value: "var(--mauve)" },
        { name: "Red", value: "var(--red)" },
        { name: "Maroon", value: "var(--maroon)" },
        { name: "Peach", value: "var(--peach)" },
        { name: "Yellow", value: "var(--yellow)" },
        { name: "Green", value: "var(--green)" },
        { name: "Teal", value: "var(--teal)" },
        { name: "Sky", value: "var(--sky)" },
        { name: "Sapphire", value: "var(--sapphire)" },
        { name: "Blue", value: "var(--blue)" },
        { name: "Lavender", value: "var(--lavender)" },
        { name: "Text", value: "var(--text)" },
        { name: "Subtext1", value: "var(--subtext1)" },
        { name: "Subtext0", value: "var(--subtext0)" },
        { name: "Overlay2", value: "var(--overlay2)" },
        { name: "Overlay1", value: "var(--overlay1)" },
        { name: "Overlay0", value: "var(--overlay0)" },
        { name: "Surface2", value: "var(--surface2)" },
        { name: "Surface1", value: "var(--surface1)" },
        { name: "Surface0", value: "var(--surface0)" },
        { name: "Base", value: "var(--base)" },
        { name: "Mantle", value: "var(--mantle)" },
        { name: "Crust", value: "var(--crust)" },
      ],
    },
  },
};

export default preview;

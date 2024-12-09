import { Meta, StoryObj } from "@storybook/react";

import SettingsDialog from ".";

/**
 * TODO: Write documentation.
 */
const meta: Meta<typeof SettingsDialog> = {
  title: "Components/Organisms/SettingsDialog",
  component: SettingsDialog,
  parameters: {
    layout: "fullscreen",
  },
};

export default meta;

type Story = StoryObj<typeof SettingsDialog>;

/**
 * TODO: Write documentation.
 */
export const Default: Story = {
  args: {
    open: true,
  },
};

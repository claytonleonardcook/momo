import { Meta, StoryObj } from "@storybook/react";

import MediaGrid from ".";

/**
 * TODO: Write documentation.
 */
const meta: Meta<typeof MediaGrid> = {
  title: "Components/Organisms/MediaGrid",
  component: MediaGrid,
  parameters: {
    layout: "fullscreen",
  },
};

export default meta;

type Story = StoryObj<typeof MediaGrid>;

/**
 * TODO: Write documentation.
 */
export const Default: Story = {};

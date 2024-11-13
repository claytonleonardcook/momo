import { Meta, StoryObj } from "@storybook/react";

import TrackControls from ".";

/**
 * TODO: Write documentation.
 */
const meta: Meta<typeof TrackControls> = {
  title: "Components/Molecules/TrackControls",
  component: TrackControls,
  parameters: {
    layout: "fullscreen",
  },
};

export default meta;

type Story = StoryObj<typeof TrackControls>;

/**
 * TODO: Write documentation.
 */
export const Default: Story = {
  args: {
    trackDuration: 100,
  },
};

import { Meta, StoryObj } from "@storybook/react";

import TrackTimeline from ".";

/**
 * TODO: Write documentation.
 */
const meta: Meta<typeof TrackTimeline> = {
  title: "Components/Molecules/TrackTimeline",
  component: TrackTimeline,
  parameters: {
    layout: "fullscreen",
  },
};

export default meta;

type Story = StoryObj<typeof TrackTimeline>;

/**
 * TODO: Write documentation.
 */
export const Default: Story = {
  render: () => {
    return (
      <div
        style={{
          padding: "16px",
        }}
      >
        <TrackTimeline />
      </div>
    );
  },
};

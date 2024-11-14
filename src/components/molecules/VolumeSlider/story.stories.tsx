import { Meta, StoryObj } from "@storybook/react";

import VolumeSlider from ".";

/**
 * TODO: Write documentation.
 */
const meta: Meta<typeof VolumeSlider> = {
  title: "Components/Molecules/VolumeSlider",
  component: VolumeSlider,
  parameters: {
    layout: "centered",
  },
};

export default meta;

type Story = StoryObj<typeof VolumeSlider>;

/**
 * TODO: Write documentation.
 */
export const Default: Story = {
  render: () => {
    return (
      <VolumeSlider
        style={{
          width: "300px",
        }}
      >
        <VolumeSlider.Icon />
        <VolumeSlider.Slider />
      </VolumeSlider>
    );
  },
};

import { Meta, StoryObj } from "@storybook/react";

import Slider from ".";
import Icon from "../Icon";
import { FaVolumeHigh } from "react-icons/fa6";

/**
 * Buttons are used to allow users to interact with the application.
 */
const meta: Meta<typeof Slider> = {
  title: "Components/Atoms/Slider",
  component: Slider,
  parameters: {
    layout: "centered",
  },
};

export default meta;

type Story = StoryObj<typeof Slider>;

/**
 * TODO: Write documentation.
 */
export const Default: Story = {
  args: {
    color: "blue",
  },
  render: (args) => {
    return (
      <Slider
        style={{
          width: "300px",
        }}
        defaultValue={50}
        {...args}
      >
        <Slider.Track />
      </Slider>
    );
  },
};

/**
 * TODO: Write documentation.
 */
export const Rainbow: Story = {
  render: () => {
    return (
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          gap: "8px",
          width: "300px",
        }}
      >
        <Slider defaultValue={15} color={"red"}>
          <Slider.Track />
        </Slider>
        <Slider defaultValue={30} color={"yellow"}>
          <Slider.Track />
        </Slider>
        <Slider defaultValue={45} color={"green"}>
          <Slider.Track />
        </Slider>
        <Slider defaultValue={60} color={"blue"}>
          <Slider.Track />
        </Slider>
        <Slider defaultValue={75} color={"mauve"}>
          <Slider.Track />
        </Slider>
        <Slider defaultValue={90} color={"lavender"}>
          <Slider.Track />
        </Slider>
      </div>
    );
  },
};

/**
 * TODO: Write documentation.
 */
export const Volume: Story = {
  args: {
    color: "blue",
  },
  render: (args) => {
    return (
      <Slider
        style={{
          width: "300px",
        }}
        defaultValue={50}
        {...args}
      >
        <Icon icon={FaVolumeHigh} />
        <Slider.Track />
      </Slider>
    );
  },
};

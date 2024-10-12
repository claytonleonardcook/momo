import { Meta, StoryObj } from "@storybook/react";

import Checkbox from ".";
import { FaSmile } from "react-icons/fa";

/**
 * Checkboxes are used to allow users to interact with binary decisions.
 */
const meta: Meta<typeof Checkbox> = {
  title: "Components/Atoms/Checkbox",
  component: Checkbox,
  parameters: {
    layout: "centered",
  },
};

export default meta;

type Story = StoryObj<typeof Checkbox>;

/**
 * TODO: Write documentation.
 */
export const Default: Story = {
  args: {
    color: "blue",
  },
  render: (args) => {
    return (
      <Checkbox {...args}>
        <Checkbox.Input />
        Checkbox
      </Checkbox>
    );
  },
};

/**
 * TODO: Write documentation.
 */
export const DifferentLayouts: Story = {
  args: {
    color: "blue",
  },
  render: (args) => {
    return (
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          gap: "8px",
        }}
      >
        <Checkbox {...args}>
          <Checkbox.Input />
          Checkbox
        </Checkbox>
        <Checkbox {...args}>
          Checkbox
          <Checkbox.Input />
        </Checkbox>
      </div>
    );
  },
};

/**
 * TODO: Write documentation.
 */
export const DifferentIcons: Story = {
  args: {
    color: "blue",
  },
  render: (args) => {
    return (
      <Checkbox {...args}>
        <Checkbox.Input icon={FaSmile} />
        Checkbox
      </Checkbox>
    );
  },
};

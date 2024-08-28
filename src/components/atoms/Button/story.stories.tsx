import { Meta, StoryObj } from "@storybook/react";

import Button from ".";

/**
 * Buttons are used to allow users to interact with the application.
 *
 * The variant of the button dictates "shape". When imagining different variants of a button disregard color and think how white-space and shape when implementing.
 */
const meta: Meta<typeof Button> = {
  title: "Components/Atoms/Button",
  component: Button,
  parameters: {
    layout: "centered",
  },
};

export default meta;

type Story = StoryObj<typeof Button>;

/**
 * TODO: Write documentation.
 */
export const Default: Story = {
  args: {
    children: "Button",
    color: "blue",
  },
};

/**
 * TODO: Write documentation.
 */
export const Standard: Story = {
  args: {
    children: "Button",
    color: "blue",
    variant: "standard",
  },
};

/**
 * TODO: Write documentation.
 */
export const Outlined: Story = {
  args: {
    children: "Button",
    color: "blue",
    variant: "outlined",
  },
};

/**
 * TODO: Write documentation.
 */
export const Text: Story = {
  args: {
    children: "Button",
    color: "blue",
    variant: "text",
  },
};

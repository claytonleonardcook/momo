import { Meta, StoryObj } from "@storybook/react";

import Card from ".";
import { ComponentProps } from "react";

/**
 * Cards are used to show users artists, albums and tracks. They're also able to be interacted with to either show more information about a artist or album or to play a track.
 */
const meta: Meta<typeof Card> = {
  title: "Components/Atoms/Card",
  component: Card,
  parameters: {
    layout: "centered",
  },
  excludeStories: /^TEST/g,
};

export default meta;

type Story = StoryObj<typeof Card>;

export const TEST_CARD: ComponentProps<typeof Card> = {
  title: "Ninja Tuna",
  subtext: "Mr. Scruff",
  image: "./ninjatuna.jpg",
};

/**
 * TODO: Write documentation.
 */
export const Default: Story = {
  args: TEST_CARD,
};

/**
 * TODO: Write documentation.
 */
export const CardGrid: Story = {
  render: () => {
    return (
      <div
        style={{
          display: "flex",
          justifyContent: "center",
        }}
      >
        <div
          style={{
            display: "grid",
            gridTemplateColumns: "repeat(4, minmax(130px, auto))",
            gap: "16px",
          }}
        >
          {new Array(8).fill(0).map((_, index) => {
            return <Card key={index} {...TEST_CARD} />;
          })}
        </div>
      </div>
    );
  },
};

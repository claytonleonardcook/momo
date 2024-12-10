import { Meta, StoryObj } from "@storybook/react";

import Toast from ".";
import Button from "../Button";
import { useToast } from "@/hooks";

const meta: Meta<typeof Toast> = {
  title: "Components/Atoms/Toast",
  component: Toast,
  parameters: {
    layout: "centered",
  },
};

export default meta;

type Story = StoryObj<typeof Toast>;

/**
 * TODO: Write documentation.
 */

export const Default: Story = {
  args: {
    message: "This is a toast!",
    color: "blue",
  },
};

/**
 * TODO: Write documentation.
 */
export const TriggeredByButton: Story = {
  render: () => {
    const { addToast } = useToast();

    return (
      <section
        style={{
          display: "flex",
          flexDirection: "row",
          flexWrap: "wrap",
          gap: "16px",
        }}
      >
        <Button
          style={{
            width: "100%",
          }}
          variant={"outlined"}
          onPress={() => addToast("This is a informational toast!", "blue")}
        >
          Informational
        </Button>
        <Button
          style={{
            width: "100%",
          }}
          variant={"outlined"}
          onPress={() => addToast("This is a warning toast!", "yellow")}
          color={"yellow"}
        >
          Warning
        </Button>
        <Button
          style={{
            width: "100%",
          }}
          variant={"outlined"}
          onPress={() => addToast("This is a danger toast!", "red")}
          color={"red"}
        >
          Danger
        </Button>
      </section>
    );
  },
};

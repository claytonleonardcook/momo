import { Meta, StoryObj } from "@storybook/react";

import Dropdown from ".";
import { FaX } from "react-icons/fa6";
import { useState } from "react";
import { Key } from "react-aria-components";

/**
 * TODO: Write documentation.
 */
const meta: Meta<typeof Dropdown> = {
  title: "Components/Atoms/Dropdown",
  component: Dropdown,
  parameters: {
    layout: "centered",
  },
};

export default meta;

type Story = StoryObj<typeof Dropdown>;

/**
 * TODO: Write documentation.
 */
export const Default: Story = {
  args: {
    color: "blue",
  },
  render: (args) => {
    const [selection, setSelection] = useState<string>("");

    const onSelectionChange = (key: Key) => {
      setSelection(key.toString());
    };

    return (
      <>
        <Dropdown {...args} onSelectionChange={onSelectionChange}>
          <Dropdown.Label>Animals</Dropdown.Label>
          <Dropdown.Button />
          <Dropdown.Options
            options={["Dog", "Cat", "Snake", "Toad", "Elephant"]}
          />
        </Dropdown>
        <p>You selected {selection ? selection : "nothing"}!</p>
      </>
    );
  },
};

/**
 * TODO: Write documentation.
 */
export const MultipleDropdowns: Story = {
  render: () => {
    const [fluidSelection, setFluidSelection] = useState<string>("milk");
    const [ouncesSelection, setOuncesSelection] = useState<string>("1");

    const onFluidSelectionChange = (key: Key) => {
      setFluidSelection(key.toString());
    };

    const onOuncesSelectionChange = (key: Key) => {
      setOuncesSelection(key.toString());
    };

    return (
      <>
        <div
          style={{
            display: "flex",
            flexDirection: "row",
            gap: "8px",
          }}
        >
          <Dropdown
            defaultSelectedKey={fluidSelection}
            onSelectionChange={onFluidSelectionChange}
            color={"sky"}
          >
            <Dropdown.Label>Fluids</Dropdown.Label>
            <Dropdown.Button />
            <Dropdown.Options options={["Water", "Juice", "Oil", "Milk"]} />
          </Dropdown>

          <Dropdown
            defaultSelectedKey={ouncesSelection}
            onSelectionChange={onOuncesSelectionChange}
            color={"lavender"}
          >
            <Dropdown.Label>Ounces</Dropdown.Label>
            <Dropdown.Button />
            <Dropdown.Options options={["1", "2", "3", "4", "5"]} />
          </Dropdown>
        </div>
        <p>
          You ordered {ouncesSelection} oz of {fluidSelection}.
        </p>
      </>
    );
  },
};

import { Meta, StoryObj } from "@storybook/react";

import RadioGroup from ".";
import { useState } from "react";

/**
 * Radio groups are used to allow users to select one option out of a list of options.
 */
const meta: Meta<typeof RadioGroup> = {
  title: "Components/Atoms/RadioGroup",
  component: RadioGroup,
  parameters: {
    layout: "centered",
  },
};

export default meta;

type Story = StoryObj<typeof RadioGroup>;

/**
 * TODO: Write documentation.
 */
export const Default: Story = {
  args: {
    color: "blue",
  },
  render: (args) => {
    const [diet, setDiet] = useState<string>("Carnivore");

    const onDietChange = setDiet;

    return (
      <>
        <RadioGroup onChange={onDietChange} defaultValue={diet} {...args}>
          <RadioGroup.RadioButton value={"Carnivore"}>
            Carnivore
          </RadioGroup.RadioButton>
          <RadioGroup.RadioButton value={"Herbivore"}>
            Herbivore
          </RadioGroup.RadioButton>
          <RadioGroup.RadioButton value={"Omnivore"}>
            Omnivore
          </RadioGroup.RadioButton>
        </RadioGroup>
        <p>You are a {diet}.</p>
      </>
    );
  },
};

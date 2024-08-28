import { Meta, StoryObj } from "@storybook/react";

const meta: Meta = {
  title: "Styles/Colors",
  tags: ["autodocs"],
};

export default meta;

type Story = StoryObj;

type ColorRowProps = {
  /**
   * Name of color in capitalized form. Ex: `Success`, `Main Text`, etc.
   */
  color: string;
};
const ColorRow = ({ color }: ColorRowProps) => {
  const variable = `--${color.toLowerCase().replace(/\s/g, "")}`;

  const onClick = () => {
    navigator.clipboard.writeText(`var(${variable})`);
  };

  return (
    <tr>
      <td>{color}</td>
      <td
        aria-label="Click to copy"
        onClick={onClick}
        style={{
          cursor: "copy",
        }}
        title="Click to copy"
      >
        <pre>{variable}</pre>
      </td>
      <td style={{ background: `var(${variable})` }}></td>
    </tr>
  );
};

/**
 * Table of colors and their corresponding CSS variables.
 *
 * Raw colors should be used sparingly and only for branding purposes.
 * Functional colors are used for text, backgrounds, and other UI elements.
 *
 * Keep in mind that all colors will change based on the current theme; but, functional colors will change more drastically.
 *
 * ***Note:* When adding a new color, make sure to add it to the proper body element in the `styles/global.scss` file. The body element should have a custom data attribute to denote which theme the variables in the body are for. Also make sure to add the new color to this table.**
 *
 * ***Note:* Use the global theme switcher to see how colors change.**
 */
export const Table: Story = {
  render: () => {
    return (
      <table
        style={{
          width: "100%",
          textAlign: "center",
        }}
      >
        <thead>
          <tr>
            <th>Color</th>
            <th>Variable</th>
            <th>Value</th>
          </tr>
        </thead>
        <tbody>
          <br />
          <td>
            <h1>Raw Colors</h1>
          </td>
          <ColorRow color="Rosewater" />
          <ColorRow color="Flamingo" />
          <ColorRow color="Pink" />
          <ColorRow color="Mauve" />
          <ColorRow color="Red" />
          <ColorRow color="Maroon" />
          <ColorRow color="Peach" />
          <ColorRow color="Yellow" />
          <ColorRow color="Green" />
          <ColorRow color="Teal" />
          <ColorRow color="Sky" />
          <ColorRow color="Sapphire" />
          <ColorRow color="Blue" />
          <ColorRow color="Lavender" />
          <br />
          <td>
            <h1>Functional Colors</h1>
          </td>
          <ColorRow color="Text" />
          <ColorRow color="Subtext1" />
          <ColorRow color="Subtext0" />
          <ColorRow color="Overlay2" />
          <ColorRow color="Overlay1" />
          <ColorRow color="Overlay0" />
          <ColorRow color="Surface2" />
          <ColorRow color="Surface1" />
          <ColorRow color="Surface0" />
          <ColorRow color="Base" />
          <ColorRow color="Mantle" />
          <ColorRow color="Crust" />
          <ColorRow color="Success" />
          <ColorRow color="Warning" />
          <ColorRow color="Danger" />
        </tbody>
      </table>
    );
  },
};

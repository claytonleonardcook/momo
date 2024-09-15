import { Meta, StoryObj } from "@storybook/react";

import Icon from ".";
import {
  FaCheck,
  FaExclamationTriangle,
  FaBrain,
  FaFile,
  FaDownload,
  FaEllipsisH,
} from "react-icons/fa";

const meta: Meta<typeof Icon> = {
  title: "Components/Atoms/Icon",
  component: Icon,
  parameters: {
    layout: "centered",
  },
};

export default meta;

type Story = StoryObj<typeof Icon>;

export const Default: Story = {
  args: {
    icon: FaBrain,
    fill: "red",
    size: 64,
  },
};

export const Grid: Story = {
  render: () => (
    <table
      style={{
        borderSpacing: "32px",
      }}
    >
      <tbody>
        <tr>
          <td>
            <Icon icon={FaCheck} fill="green" size={64} />
          </td>
          <td>
            <Icon icon={FaExclamationTriangle} fill="yellow" size={64} />
          </td>
          <td>
            <Icon icon={FaBrain} fill="red" size={64} />
          </td>
        </tr>
        <tr>
          <td>
            <Icon icon={FaFile} fill="blue" size={64} />
          </td>
          <td>
            <Icon icon={FaDownload} fill="green" size={64} />
          </td>
          <td>
            <Icon icon={FaEllipsisH} fill="red" size={64} />
          </td>
        </tr>
      </tbody>
    </table>
  ),
};

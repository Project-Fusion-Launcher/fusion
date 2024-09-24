import type { Meta, StoryObj } from "storybook-solidjs";
import Select from "../components/Select";
import "../index.css";

const meta = {
  title: "Select",
  component: Select,
  tags: ["autodocs"],
} satisfies Meta<typeof Select>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Primary: Story = {
  args: {
    options: ["Option 1", "Option 2", "Option 3"],
    placeholder: "Select an option",
    variant: "primary",
  },
};

export const Secondary: Story = {
  args: {
    options: ["Option 1", "Option 2", "Option 3"],
    placeholder: "Select an option",
    variant: "secondary",
  },
};

export const Outline: Story = {
  args: {
    options: ["Option 1", "Option 2", "Option 3"],
    placeholder: "Select an option",
    variant: "outline",
  },
};

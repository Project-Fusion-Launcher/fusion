import type { Meta, StoryObj } from "storybook-solidjs";
import TextField from "../components/TextField";
import "../index.pcss";

// More on how to set up stories at: https://storybook.js.org/docs/7.0/solid/writing-stories/introduction
const meta = {
  title: "TextField",
  component: TextField,
  tags: ["autodocs"],
} satisfies Meta<typeof TextField>;

export default meta;
type Story = StoryObj<typeof meta>;

// More on writing stories with args: https://storybook.js.org/docs/7.0/solid/writing-stories/args
export const Default: Story = {
  args: {
    size: "large",
    variant: "default",
  },
};

export const Outline: Story = {
  args: {
    size: "large",
    variant: "outline",
  },
};

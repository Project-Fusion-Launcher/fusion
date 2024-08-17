import type { Meta, StoryObj } from "storybook-solidjs";
import TextField from "../components/TextField";
import "../index.pcss";

const meta = {
  title: "TextField",
  component: TextField,
  tags: ["autodocs"],
} satisfies Meta<typeof TextField>;

export default meta;
type Story = StoryObj<typeof meta>;

export const DefaultDefault: Story = {
  args: {
    size: "default",
    variant: "default",
  },
};

export const DefaultOutline: Story = {
  args: {
    size: "default",
    variant: "outline",
  },
};

export const LargeDefault: Story = {
  args: {
    size: "large",
    variant: "default",
  },
};

export const LargeOutline: Story = {
  args: {
    size: "large",
    variant: "outline",
  },
};

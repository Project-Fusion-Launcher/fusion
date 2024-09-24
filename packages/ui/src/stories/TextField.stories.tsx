import type { Meta, StoryObj } from "storybook-solidjs";
import TextField from "../components/TextField";
import "../index.css";

const meta = {
  title: "TextField",
  component: TextField,
  tags: ["autodocs"],
} satisfies Meta<typeof TextField>;

export default meta;
type Story = StoryObj<typeof meta>;

export const DefaultMedium: Story = {
  args: {
    size: "md",
    variant: "default",
  },
};

export const OutlineMedium: Story = {
  args: {
    size: "md",
    variant: "outline",
  },
};

export const DefaultLarge: Story = {
  args: {
    size: "lg",
    variant: "default",
  },
};

export const OutlineLarge: Story = {
  args: {
    size: "lg",
    variant: "outline",
  },
};

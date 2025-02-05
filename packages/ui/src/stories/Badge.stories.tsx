import type { Meta, StoryObj } from "storybook-solidjs";
import Badge from "../components/Badge";
import "../index.css";

const meta = {
  title: "Badge",
  component: Badge,
  tags: ["autodocs"],
} satisfies Meta<typeof Badge>;

export default meta;
type Story = StoryObj<typeof meta>;

export const PrimaryMedium: Story = {
  args: {
    variant: "primary",
    size: "md",
    children: "Badge",
    round: true,
  },
};

export const SecondaryMedium: Story = {
  args: {
    variant: "secondary",
    size: "md",
    children: "Badge",
  },
};

export const AccentMedium: Story = {
  args: {
    variant: "accent",
    size: "md",
    children: "Badge",
  },
};

export const OutlineMedium: Story = {
  args: {
    variant: "outline",
    size: "md",
    children: "Badge",
  },
};

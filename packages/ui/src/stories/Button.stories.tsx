import type { Meta, StoryObj } from "storybook-solidjs";
import Button from "../components/Button";
import "../index.css";

const meta = {
  title: "Button",
  component: Button,
  tags: ["autodocs"],
} satisfies Meta<typeof Button>;

export default meta;
type Story = StoryObj<typeof meta>;

export const PrimaryMedium: Story = {
  args: {
    variant: "primary",
    size: "md",
    children: () => <span>Button</span>,
  },
};

export const SecondaryMedium: Story = {
  args: {
    variant: "secondary",
    size: "md",
    children: () => <span>Button</span>,
  },
};

export const AccentMedium: Story = {
  args: {
    variant: "accent",
    size: "md",
    children: () => <span>Button</span>,
  },
};

export const OutlineMedium: Story = {
  args: {
    variant: "outline",
    size: "md",
    children: () => <span>Button</span>,
  },
};

export const GhostMedium: Story = {
  args: {
    variant: "ghost",
    size: "md",
    children: () => <span>Button</span>,
  },
};

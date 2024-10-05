import type { Meta, StoryObj } from "storybook-solidjs";
import IconButton from "../components/IconButton";
import "../index.css";
import { Folder } from "lucide-solid";

const meta = {
  title: "IconButton",
  component: IconButton,
  tags: ["autodocs"],
} satisfies Meta<typeof IconButton>;

export default meta;
type Story = StoryObj<typeof meta>;

export const PrimaryMedium: Story = {
  args: {
    variant: "primary",
    size: "md",
    children: () => <Folder />,
  },
};

export const SecondaryMedium: Story = {
  args: {
    variant: "secondary",
    size: "md",
    children: () => <Folder />,
  },
};

export const AccentMedium: Story = {
  args: {
    variant: "accent",
    size: "md",
    children: () => <Folder />,
  },
};

export const OutlineMedium: Story = {
  args: {
    variant: "outline",
    size: "md",
    children: () => <Folder />,
  },
};

export const GhostMedium: Story = {
  args: {
    variant: "ghost",
    size: "md",
    children: () => <Folder />,
  },
};

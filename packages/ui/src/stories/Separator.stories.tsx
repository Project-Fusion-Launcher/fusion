import type { Meta, StoryObj } from "storybook-solidjs";
import Separator from "../components/Separator";
import "../index.pcss";

const meta = {
  title: "Separator",
  component: Separator,
  tags: ["autodocs"],
} satisfies Meta<typeof Separator>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Full: Story = {
  args: {
    width: "full",
  },
};

export const Half: Story = {
  args: {
    width: "50",
  },
};

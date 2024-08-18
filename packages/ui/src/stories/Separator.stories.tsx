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

export const Default: Story = {
  args: {
    width: "default",
  },
};

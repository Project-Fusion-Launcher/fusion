import type { Meta, StoryObj } from "storybook-solidjs";
import Dialog from "../components/Dialog";
import "../index.css";

const meta = {
  title: "Dialog",
  component: Dialog,
  tags: ["autodocs"],
} satisfies Meta<typeof Dialog>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    title: "Dialog Title",
    defaultOpen: true,
  },
};

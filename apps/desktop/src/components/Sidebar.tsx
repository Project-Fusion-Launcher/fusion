import { Tabs } from "@kobalte/core";

const Sidebar = () => {
  return (
    <Tabs.Root
      orientation="vertical"
      class="bg-background border-border h-full w-72 border-r"
    >
      <Tabs.List />
    </Tabs.Root>
  );
};

export default Sidebar;

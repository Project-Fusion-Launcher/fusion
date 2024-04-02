import { Tabs } from "@kobalte/core";

const Sidebar = () => {
  return (
    <Tabs.Root
      orientation="vertical"
      class="h-full w-72 bg-background border-r border-border"
    >
      <Tabs.List />
    </Tabs.Root>
  );
};

export default Sidebar;

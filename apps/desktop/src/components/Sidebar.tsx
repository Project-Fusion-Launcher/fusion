import { Tabs } from "@kobalte/core";
import { Separator } from "@kobalte/core/separator";
import { Box, Boxes, Joystick, Library, Store, UsersRound } from "lucide-solid";
import { For } from "solid-js";

interface SidebarProps {
  currentTab: string;
  onTabChange: (tab: string) => void;
}

const pagesSection1 = [
  { name: "library", icon: Library },
  { name: "retro", icon: Joystick },
  { name: "collections", icon: Boxes },
];

const pagesSection2 = [
  { name: "stores", icon: Store },
  { name: "friends", icon: UsersRound },
];

const Sidebar = (props: SidebarProps) => {
  return (
    <Tabs.Root
      orientation="vertical"
      class="bg-background border-border z-50 flex h-full w-72 flex-shrink-0 flex-col items-center border-r"
      value={props.currentTab}
      onChange={props.onTabChange}
    >
      <div class="flex items-center py-44">
        <Box class="text-primary size-48" style={{ "stroke-width": "2px" }} />
      </div>
      <Tabs.List class="text-primary relative flex w-full flex-col items-center">
        <For each={pagesSection1}>
          {(page) => (
            <Tabs.Trigger
              value={page.name}
              class="relative flex h-52 items-center justify-center"
            >
              <page.icon class="size-32" />
            </Tabs.Trigger>
          )}
        </For>
        <Separator class="bg-border m-8 h-1 w-32 border-0" />
        <For each={pagesSection2}>
          {(page) => (
            <Tabs.Trigger
              value={page.name}
              class="relative flex h-52 items-center justify-center"
            >
              <page.icon class="size-32" />
            </Tabs.Trigger>
          )}
        </For>
        <Tabs.Indicator class="bg-primary background border-r-md border-accent absolute w-full bg-opacity-5 transition-transform" />
      </Tabs.List>
    </Tabs.Root>
  );
};

export default Sidebar;

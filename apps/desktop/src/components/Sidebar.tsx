import { Tabs } from "@kobalte/core";
import { Separator } from "@kobalte/core/separator";
import { Box, Boxes, Joystick, Library, Store, UsersRound } from "lucide-solid";
import { For } from "solid-js";

const pagesSection1 = [
  { name: "library", icon: Library },
  { name: "retro", icon: Joystick },
  { name: "collections", icon: Boxes },
];

const pagesSection2 = [
  { name: "stores", icon: Store },
  { name: "friends", icon: UsersRound },
];

const Sidebar = () => {
  return (
    <Tabs.Root
      orientation="vertical"
      class="bg-background border-border flex h-full w-72 flex-col items-center border-r"
    >
      <div class="h-136 flex items-center">
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
        <Separator class="bg-border m-8 h-[2px] w-1/2 border-0" />
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

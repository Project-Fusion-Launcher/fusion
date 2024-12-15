import { Tabs } from "@kobalte/core";
import { Separator } from "@repo/ui";
import { Box } from "lucide-solid";
import { For, Show } from "solid-js";
import { pages } from "../models/pages";

interface SidebarProps {
  currentTab: string;
  onTabChange: (tab: string) => void;
}

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
      <Tabs.List class="text-secondary relative flex w-full flex-col items-center">
        <For each={pages}>
          {(page) => (
            <>
              <Show when={page.name === "storefronts"}>
                <Separator />
              </Show>
              <Tabs.Trigger
                value={page.name}
                class="relative flex h-52 w-full items-center justify-center"
              >
                <page.icon
                  class="size-32 transition-all"
                  classList={{
                    "stroke-white scale-105": page.name == props.currentTab,
                  }}
                />
              </Tabs.Trigger>
            </>
          )}
        </For>
        <Tabs.Indicator class="border-r-md border-accent absolute w-full transition-transform" />
      </Tabs.List>
    </Tabs.Root>
  );
};

export default Sidebar;

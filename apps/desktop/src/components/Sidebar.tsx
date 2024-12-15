import { Tabs } from "@kobalte/core";
import { Separator } from "@repo/ui";
import { Box } from "lucide-solid";
import type { Page } from "../models/pages";
import { pages } from "../models/pages";
import { Dynamic } from "solid-js/web";

interface SidebarTriggerProps {
  page: Page;
  currentTab: string;
  disabled?: boolean;
}

const SidebarTrigger = (props: SidebarTriggerProps) => {
  return (
    <Tabs.Trigger
      value={props.page.name}
      class="relative flex h-52 w-full items-center justify-center"
      disabled={props.disabled}
    >
      <Dynamic
        component={props.page.icon}
        // @ts-ignore
        class="size-32 transition-all"
        classList={{
          "stroke-white scale-105": props.page.name == props.currentTab,
        }}
      />
    </Tabs.Trigger>
  );
};

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
      <Tabs.List class="text-secondary relative flex h-full w-full flex-col items-center">
        <SidebarTrigger page={pages.library} currentTab={props.currentTab} />
        <SidebarTrigger page={pages.retro} currentTab={props.currentTab} />
        <SidebarTrigger
          page={pages.collections}
          currentTab={props.currentTab}
          disabled
        />
        <Separator />
        <SidebarTrigger page={pages.downloads} currentTab={props.currentTab} />
        <SidebarTrigger
          page={pages.storefronts}
          currentTab={props.currentTab}
          disabled
        />
        <SidebarTrigger
          page={pages.friends}
          currentTab={props.currentTab}
          disabled
        />
        <Tabs.Indicator class="border-r-md border-accent absolute w-full transition-transform" />
      </Tabs.List>
    </Tabs.Root>
  );
};

export default Sidebar;

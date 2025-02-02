import { Tabs } from "@kobalte/core";
import { Separator } from "@repo/ui";
import { Box } from "lucide-solid";
import type { Page } from "../models/pages";
import { pages } from "../models/pages";
import { Dynamic } from "solid-js/web";
import { createResource } from "solid-js";
import { getVersion } from "@tauri-apps/api/app";

interface SidebarTriggerProps {
  page: Page;
  selectedTab: string;
  disabled?: boolean;
  moveBottom?: boolean;
}

const SidebarTrigger = (props: SidebarTriggerProps) => {
  return (
    <Tabs.Trigger
      value={props.page.name}
      class="relative flex h-52 w-full items-center justify-center"
      classList={{ "mt-auto  ": props.moveBottom }}
      disabled={props.disabled}
    >
      <Dynamic
        component={props.page.icon}
        class="size-32 transition-all"
        classList={{
          "stroke-white scale-105": props.page.name == props.selectedTab,
        }}
      />
    </Tabs.Trigger>
  );
};

interface SidebarProps {
  selectedTab: string;
  onTabChange: (tab: string) => void;
}

const Sidebar = (props: SidebarProps) => {
  const [appVersion] = createResource(getVersion);

  return (
    <Tabs.Root
      orientation="vertical"
      class="bg-background border-border z-50 flex h-full w-72 flex-shrink-0 flex-col items-center border-r"
      value={props.selectedTab}
      onChange={props.onTabChange}
    >
      <div class="flex items-center py-44">
        <Box class="text-primary size-48" style={{ "stroke-width": "2px" }} />
      </div>
      <Tabs.List class="text-secondary relative mb-32 flex h-full w-full flex-col items-center">
        <SidebarTrigger page={pages.library} selectedTab={props.selectedTab} />
        <SidebarTrigger page={pages.retro} selectedTab={props.selectedTab} />
        <SidebarTrigger
          page={pages.collections}
          selectedTab={props.selectedTab}
          disabled
        />
        <Separator />
        <SidebarTrigger
          page={pages.downloads}
          selectedTab={props.selectedTab}
        />
        <SidebarTrigger
          page={pages.storefronts}
          selectedTab={props.selectedTab}
          disabled
        />
        <SidebarTrigger
          page={pages.friends}
          selectedTab={props.selectedTab}
          disabled
        />
        <SidebarTrigger
          page={pages.settings}
          selectedTab={props.selectedTab}
          moveBottom
        />
        <Tabs.Indicator class="border-r-md border-accent absolute w-full transition-transform" />
      </Tabs.List>
      <span class="text-secondary absolute bottom-8 text-sm">
        {appVersion()}
      </span>
    </Tabs.Root>
  );
};

export default Sidebar;

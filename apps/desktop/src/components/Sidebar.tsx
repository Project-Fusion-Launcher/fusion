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
  currentTab: string;
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
  const [appVersion] = createResource(getVersion);

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
      <Tabs.List class="text-secondary relative mb-32 flex h-full w-full flex-col items-center">
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
        <SidebarTrigger
          page={pages.settings}
          currentTab={props.currentTab}
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

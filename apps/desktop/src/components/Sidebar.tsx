import { Tabs } from "@kobalte/core";
import { Separator } from "@repo/ui";
import { Box } from "lucide-solid";
import type { Page } from "../models/pages";
import { pages } from "../models/pages";
import { Dynamic } from "solid-js/web";
import { createMemo, createResource } from "solid-js";
import { getVersion } from "@tauri-apps/api/app";
import { useLocation, useNavigate } from "@solidjs/router";

interface SidebarTriggerProps {
  page: Page;
  selectedTab: string;
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
        class="size-32 transition-all"
        classList={{
          "stroke-white scale-105": props.page.name == props.selectedTab,
        }}
      />
    </Tabs.Trigger>
  );
};

const Sidebar = () => {
  const [appVersion] = createResource(getVersion);

  const location = useLocation();
  const navigate = useNavigate();

  const selectedTab = createMemo(
    () => location.pathname.split("/")[1] || pages.library.name,
  );

  function changeTab(tab: string) {
    navigate("/" + tab);
  }

  return (
    <Tabs.Root
      orientation="vertical"
      class="bg-background border-border z-50 flex h-full w-72 shrink-0 flex-col items-center border-r"
      value={selectedTab()}
      onChange={changeTab}
    >
      <div class="flex items-center py-44">
        <Box class="text-primary size-48" style={{ "stroke-width": "2px" }} />
      </div>
      <Tabs.List class="text-secondary relative mb-32 flex h-full w-full flex-col items-center">
        <SidebarTrigger page={pages.library} selectedTab={selectedTab()} />
        <SidebarTrigger page={pages.retro} selectedTab={selectedTab()} />
        <SidebarTrigger
          page={pages.collections}
          selectedTab={selectedTab()}
          disabled
        />
        <Separator class="my-8 w-1/2" />
        <SidebarTrigger page={pages.downloads} selectedTab={selectedTab()} />
        <SidebarTrigger
          page={pages.storefronts}
          selectedTab={selectedTab()}
          disabled
        />
        <SidebarTrigger
          page={pages.friends}
          selectedTab={selectedTab()}
          disabled
        />
        <div class="mt-auto w-full">
          <SidebarTrigger page={pages.settings} selectedTab={selectedTab()} />
        </div>
        <Tabs.Indicator class="border-accent absolute w-full border-r-2 transition-transform" />
      </Tabs.List>
      <span class="text-secondary absolute bottom-8 text-xs">
        {appVersion()}
      </span>
    </Tabs.Root>
  );
};

export default Sidebar;

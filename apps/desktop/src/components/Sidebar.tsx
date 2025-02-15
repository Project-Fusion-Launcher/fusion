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
  disabled?: boolean;
}

const SidebarTrigger = (props: SidebarTriggerProps) => {
  return (
    <Tabs.Trigger
      value={props.page.name}
      class="relative flex h-52 w-full items-center justify-center data-[selected]:[&>*]:scale-105 data-[selected]:[&>*]:stroke-white"
      disabled={props.disabled}
    >
      <Dynamic component={props.page.icon} class="size-32 transition-all" />
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
        <SidebarTrigger page={pages.library} />
        <SidebarTrigger page={pages.retro} />
        <SidebarTrigger page={pages.collections} disabled />
        <Separator class="my-8 w-1/2" />
        <SidebarTrigger page={pages.downloads} />
        <SidebarTrigger page={pages.storefronts} disabled />
        <SidebarTrigger page={pages.friends} disabled />
        <div class="mt-auto w-full">
          <SidebarTrigger page={pages.settings} />
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

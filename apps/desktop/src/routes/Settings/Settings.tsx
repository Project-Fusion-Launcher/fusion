import Header from "../../components/Header";
import { createMemo } from "solid-js";
import { Dynamic } from "solid-js/web";
import { Tabs, TabsIndicator, TabsList, TabsTrigger } from "@repo/ui";
import type { RouteSectionProps } from "@solidjs/router";
import { useLocation, useNavigate } from "@solidjs/router";
import { settingsTabs } from "../../models/pages";
import { capitalizeFirstLetter } from "../../util/string";
import { For } from "solid-js";

const Settings = (props: RouteSectionProps) => {
  const location = useLocation();
  const navigate = useNavigate();

  const selectedTab = createMemo(
    () => location.pathname.split("/")[2] || "storefronts",
  );

  function changeTab(tab: string) {
    navigate(tab);
  }

  return (
    <>
      <Header title="Settings" hideSearch />
      <div class="px-40">
        <Tabs value={selectedTab()} onChange={changeTab}>
          <TabsList>
            <For each={Object.values(settingsTabs)}>
              {(tab) => (
                <TabsTrigger value={tab.name}>
                  <span class="flex items-center gap-8">
                    <Dynamic component={tab.icon} class="size-20" />
                    {capitalizeFirstLetter(tab.name)}
                  </span>
                </TabsTrigger>
              )}
            </For>
            <TabsIndicator />
          </TabsList>
        </Tabs>
      </div>
      <div class="p-40">{props.children}</div>
    </>
  );
};

export default Settings;

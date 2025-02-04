import Header from "../../components/Header";
import { createMemo } from "solid-js";
import { Dynamic } from "solid-js/web";
import { Tabs } from "@repo/ui";
import type { RouteSectionProps } from "@solidjs/router";
import { useLocation, useNavigate } from "@solidjs/router";
import { settingsTabs } from "../../models/pages";
import { capitalizeFirstLetter } from "../../util/string";

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
        <Tabs
          values={[
            settingsTabs.storefronts.name,
            settingsTabs.general.name,
            settingsTabs.downloads.name,
          ]}
          value={selectedTab()}
          onChange={changeTab}
          indicator
        >
          <span class="flex items-center gap-8">
            <Dynamic
              component={settingsTabs.storefronts.icon}
              class="size-20"
            />
            {capitalizeFirstLetter(settingsTabs.storefronts.name)}
          </span>
          <span class="flex items-center gap-8">
            <Dynamic component={settingsTabs.general.icon} class="size-20" />
            {capitalizeFirstLetter(settingsTabs.general.name)}
          </span>
          <span class="flex items-center gap-8">
            <Dynamic component={settingsTabs.downloads.icon} class="size-20" />
            {capitalizeFirstLetter(settingsTabs.downloads.name)}
          </span>
        </Tabs>
      </div>
      <div class="p-40">{props.children}</div>
    </>
  );
};

export default Settings;

import Header from "../../components/Header";
import { createSignal } from "solid-js";
import { Dynamic } from "solid-js/web";
import { Download, Globe, Store } from "lucide-solid";
import { Tabs } from "@repo/ui";
import type { RouteSectionProps } from "@solidjs/router";
import { useNavigate } from "@solidjs/router";

const Settings = (props: RouteSectionProps) => {
  const [selectedTab, setSelectedTab] = createSignal("storefronts");

  const navigate = useNavigate();

  function onTabChange(tab: string) {
    console.log(tab);
    setSelectedTab(tab);
    navigate(`${tab}`);
  }

  return (
    <>
      <Header title="Settings" hideSearch />
      <div class="px-40">
        <Tabs
          values={["storefronts", "general", "downloads"]}
          value={selectedTab()}
          onChange={onTabChange}
          indicator
        >
          <span class="flex items-center gap-8">
            <Dynamic component={Store} class="size-20" />
            Storefronts
          </span>
          <span class="flex items-center gap-8">
            <Dynamic component={Globe} class="size-20" />
            General
          </span>
          <span class="flex items-center gap-8">
            <Dynamic component={Download} class="size-20" />
            Downloads
          </span>
        </Tabs>
      </div>
      <div class="p-40">{props.children}</div>
    </>
  );
};

export default Settings;

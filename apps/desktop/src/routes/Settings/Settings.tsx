import { Tabs } from "@kobalte/core/tabs";
import Header from "../../components/Header";
import { createSignal, type Component } from "solid-js";
import { Dynamic } from "solid-js/web";
import { Download, Globe, Store } from "lucide-solid";
import { Button } from "@kobalte/core/button";

interface SettingsTriggerProps {
  value: string;
  name: string;
  icon: Component;
  selectedTab: string;
}

const SettingsTrigger = (props: SettingsTriggerProps) => {
  return (
    <Tabs.Trigger
      value={props.value}
      class="flex items-center gap-8 transition-colors"
      classList={{
        "text-primary": props.selectedTab === props.value,
      }}
    >
      <Dynamic
        component={props.icon}
        // @ts-ignore
        class="size-20"
      />
      {props.name}
    </Tabs.Trigger>
  );
};

const Settings = () => {
  const [selectedTab, setSelectedTab] = createSignal("storefronts");

  return (
    <>
      <Header title="Settings" hideSearch />
      <div class="px-40">
        <Tabs
          orientation="horizontal"
          class="flex flex-col"
          value={selectedTab()}
          onChange={setSelectedTab}
        >
          <Tabs.List class="text-secondary relative mb-40 flex h-28 w-min flex-row justify-start gap-40 text-left font-medium">
            <SettingsTrigger
              value="storefronts"
              name="Storefronts"
              icon={Store}
              selectedTab={selectedTab()}
            />
            <Tabs.Indicator class="border-b-md border-accent absolute -bottom-1 z-20 h-full w-full transition-all" />
            <SettingsTrigger
              value="general"
              name="General"
              icon={Globe}
              selectedTab={selectedTab()}
            />
            <SettingsTrigger
              value="downloads"
              name="Downloads"
              icon={Download}
              selectedTab={selectedTab()}
            />
          </Tabs.List>
          <Tabs.Content value="storefronts" class="text-primary w-full">
            <div class="flex gap-16">
              <Button class="h-136 w-136 border-border border-sm rounded bg-gray-200 p-4">
                itch.io
              </Button>
              <Button class="h-136 w-136 border-border border-sm rounded bg-gray-200 p-4">
                Legacy Games
              </Button>
            </div>
          </Tabs.Content>
          <Tabs.Content value="general" />
          <Tabs.Content value="downloads" />
        </Tabs>
      </div>
    </>
  );
};

export default Settings;

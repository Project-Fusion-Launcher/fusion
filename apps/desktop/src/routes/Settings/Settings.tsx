import { Tabs } from "@kobalte/core/tabs";
import Header from "../../components/Header";
import { createSignal, type Component } from "solid-js";
import { Dynamic } from "solid-js/web";
import { Download, Globe, Store } from "lucide-solid";

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
      <Tabs
        orientation="vertical"
        class="flex"
        value={selectedTab()}
        onChange={setSelectedTab}
      >
        <Tabs.List class="text-secondary relative flex flex-col justify-start gap-20 border-r px-40 text-left font-medium">
          <SettingsTrigger
            value="storefronts"
            name="Storefronts"
            icon={Store}
            selectedTab={selectedTab()}
          />
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
        <Tabs.Content value="storefronts" class="text-primary px-40" />
        <Tabs.Content value="general" class="px-40" />
        <Tabs.Content value="downloads" class="text-primary" />
      </Tabs>
    </>
  );
};

export default Settings;

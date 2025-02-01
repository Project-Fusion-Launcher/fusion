import { Tabs as KTabs } from "@kobalte/core/tabs";
import type { JSXElement } from "solid-js";
import { For, Show } from "solid-js";

interface TabsProps {
  value?: string;
  onChange?: (value: string) => void;
  indicator?: boolean;
  children: JSXElement[];
  values: string[];
}

const Tabs = (props: TabsProps) => {
  return (
    <KTabs
      orientation="horizontal"
      class="flex h-28 flex-col"
      value={props.value}
      onChange={props.onChange}
    >
      <KTabs.List class="text-secondary relative mb-40 flex w-min flex-row justify-start gap-40 text-left font-medium">
        <For each={props.children}>
          {(tab, i) => (
            <KTabs.Trigger
              value={props.values[i()]}
              class="flex items-center gap-8 transition-colors"
              classList={{
                "text-primary": props.value === props.values[i()],
              }}
            >
              {tab}
            </KTabs.Trigger>
          )}
        </For>
        <Show when={props.indicator}>
          <KTabs.Indicator class="border-b-md border-accent absolute -bottom-8 z-20 h-full w-full transition-all" />
        </Show>
      </KTabs.List>
    </KTabs>
  );
};

export default Tabs;

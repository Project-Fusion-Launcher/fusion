/*import { Tabs as KTabs } from "@kobalte/core/tabs";
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
      class="flex flex-col"
      value={props.value}
      onChange={props.onChange}
    >
      <KTabs.List class="text-secondary relative flex h-28 w-min flex-row justify-start gap-40 text-left font-medium">
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
          <KTabs.Indicator class="border-accent absolute -bottom-8 z-20 h-full w-full border-b-2 transition-all" />
        </Show>
      </KTabs.List>
    </KTabs>
  );
};

export default Tabs;*/

import type { PolymorphicProps } from "@kobalte/core";
import * as TabsPrimitive from "@kobalte/core/tabs";
import type { ValidComponent } from "solid-js";
import { splitProps } from "solid-js";
import { cn } from "../utils";

const Tabs = TabsPrimitive.Root;

type TabsListProps<T extends ValidComponent = "div"> =
  TabsPrimitive.TabsListProps<T> & {
    class?: string | undefined;
  };

const TabsList = <T extends ValidComponent = "div">(
  props: PolymorphicProps<T, TabsListProps<T>>,
) => {
  const [local, others] = splitProps(props as TabsListProps, ["class"]);
  return (
    <TabsPrimitive.List
      class={cn(
        "text-secondary relative flex h-28 w-min flex-row justify-start gap-40 text-left font-medium",
        local.class,
      )}
      {...others}
    />
  );
};

type TabsTriggerProps<T extends ValidComponent = "button"> =
  TabsPrimitive.TabsTriggerProps<T> & {
    class?: string | undefined;
  };

const TabsTrigger = <T extends ValidComponent = "button">(
  props: PolymorphicProps<T, TabsTriggerProps<T>>,
) => {
  const [local, others] = splitProps(props as TabsTriggerProps, ["class"]);
  return (
    <TabsPrimitive.Trigger
      class={cn(
        "data-[selected]:text-primary flex items-center gap-8 transition-colors",
        local.class,
      )}
      {...others}
    />
  );
};

type TabsContentProps<T extends ValidComponent = "div"> =
  TabsPrimitive.TabsContentProps<T> & {
    class?: string | undefined;
  };

const TabsContent = <T extends ValidComponent = "div">(
  props: PolymorphicProps<T, TabsContentProps<T>>,
) => {
  const [local, others] = splitProps(props as TabsContentProps, ["class"]);
  return <TabsPrimitive.Content class={local.class} {...others} />;
};

type TabsIndicatorProps<T extends ValidComponent = "div"> =
  TabsPrimitive.TabsIndicatorProps<T> & {
    class?: string | undefined;
  };

const TabsIndicator = <T extends ValidComponent = "div">(
  props: PolymorphicProps<T, TabsIndicatorProps<T>>,
) => {
  const [local, others] = splitProps(props as TabsIndicatorProps, ["class"]);
  return (
    <TabsPrimitive.Indicator
      class={cn(
        "border-accent absolute -bottom-8 z-20 h-full w-full border-b-2 transition-all",
        local.class,
      )}
      {...others}
    />
  );
};

export { Tabs, TabsList, TabsTrigger, TabsContent, TabsIndicator };

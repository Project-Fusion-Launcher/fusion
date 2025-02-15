import type { JSXElement, ValidComponent } from "solid-js";
import { splitProps } from "solid-js";
import type { PolymorphicProps } from "@kobalte/core/polymorphic";
import * as SelectPrimitive from "@kobalte/core/select";
import { cn } from "../utils";
import { Check, ChevronsUpDown } from "lucide-solid";

const SelectValue = SelectPrimitive.Value;
const SelectHiddenSelect = SelectPrimitive.HiddenSelect;

type SelectProps<
  Option,
  OptGroup = never,
  T extends ValidComponent = "div",
> = SelectPrimitive.SelectRootProps<Option, OptGroup, T> & {
  class?: string | undefined;
  children?: JSXElement;
};

const Select = <Option, OptGroup = never, T extends ValidComponent = "div">(
  props: PolymorphicProps<T, SelectProps<Option, OptGroup, T>>,
) => {
  const [local, others] = splitProps(
    props as SelectProps<Option, OptGroup, T>,
    ["class", "children"],
  );
  return (
    <SelectPrimitive.Root
      class={cn("flex flex-col gap-12", local.class)}
      {...others}
    >
      {local.children}
    </SelectPrimitive.Root>
  );
};

type SelectTriggerProps<T extends ValidComponent = "button"> =
  SelectPrimitive.SelectTriggerProps<T> & {
    class?: string | undefined;
    children?: JSXElement;
  };

const SelectTrigger = <T extends ValidComponent = "button">(
  props: PolymorphicProps<T, SelectTriggerProps<T>>,
) => {
  const [local, others] = splitProps(props as SelectTriggerProps, [
    "class",
    "children",
  ]);
  return (
    <SelectPrimitive.Trigger
      class={cn(
        "bg-popover border-border border-1 text-popover-foreground flex h-40 w-full items-center gap-8 rounded-md px-16 disabled:cursor-not-allowed disabled:opacity-50",
        local.class,
      )}
      {...others}
    >
      {local.children}
      <SelectPrimitive.Icon as={ChevronsUpDown} class="ml-auto h-16 w-auto" />
    </SelectPrimitive.Trigger>
  );
};

type SelectLabelProps<T extends ValidComponent = "span"> =
  SelectPrimitive.SelectLabelProps<T> & {
    class?: string | undefined;
    children?: JSXElement;
  };

const SelectLabel = <T extends ValidComponent = "span">(
  props: PolymorphicProps<T, SelectLabelProps<T>>,
) => {
  const [local, others] = splitProps(props as SelectLabelProps, [
    "class",
    "children",
  ]);
  return (
    <SelectPrimitive.Label
      class={cn("text-secondary text-base font-light", local.class)}
      {...others}
    >
      {local.children}
    </SelectPrimitive.Label>
  );
};

type SelectContentProps<T extends ValidComponent = "div"> =
  SelectPrimitive.SelectContentProps<T> & { class?: string | undefined };

const SelectContent = <T extends ValidComponent = "div">(
  props: PolymorphicProps<T, SelectContentProps<T>>,
) => {
  const [local, others] = splitProps(props as SelectContentProps, ["class"]);
  return (
    <SelectPrimitive.Portal>
      <SelectPrimitive.Content
        class={cn(
          "bg-popover popover-foreground data-[expanded]:animate-in border-1 border-border relative z-50 min-w-32 origin-[var(--kb-select-content-transform-origin)] overflow-hidden rounded-md p-4 text-sm shadow-md",
          local.class,
        )}
        {...others}
      >
        <SelectPrimitive.Listbox class="m-0 p-1" />
      </SelectPrimitive.Content>
    </SelectPrimitive.Portal>
  );
};

type SelectItemProps<T extends ValidComponent = "li"> =
  SelectPrimitive.SelectItemProps<T> & {
    class?: string | undefined;
    children?: JSXElement;
  };

const SelectItem = <T extends ValidComponent = "li">(
  props: PolymorphicProps<T, SelectItemProps<T>>,
) => {
  const [local, others] = splitProps(props as SelectItemProps, [
    "class",
    "children",
  ]);
  return (
    <SelectPrimitive.Item
      class={cn(
        "text-popover-foreground hover:bg-highlight flex h-32 cursor-pointer items-center gap-8 rounded-sm px-8",
        local.class,
      )}
      {...others}
    >
      <SelectPrimitive.ItemLabel class="text-primary font-medium">
        {local.children}
      </SelectPrimitive.ItemLabel>
      <SelectPrimitive.ItemIndicator as={Check} class="ml-auto h-16 w-auto" />
    </SelectPrimitive.Item>
  );
};

export {
  Select,
  SelectValue,
  SelectHiddenSelect,
  SelectTrigger,
  SelectContent,
  SelectItem,
  SelectLabel,
};

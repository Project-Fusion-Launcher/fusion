/*import { Select as KSelect } from "@kobalte/core/select";
import { Check, ChevronsUpDown, LoaderCircle } from "lucide-solid";
import type { VariantProps } from "tailwind-variants";
import { tv } from "tailwind-variants";
import "./styles.css";
import { Show } from "solid-js";

const select = tv({
  slots: {
    trigger: "min-w-136 flex h-40 items-center gap-8 rounded-md px-16",
    portal: "overflow-hidden rounded-md p-8",
  },
  variants: {
    variant: {
      primary: {
        trigger: "bg-primary",
        portal: "bg-primary",
      },
      secondary: {
        trigger: "bg-secondary",
        portal: "bg-secondary",
      },
      outline: {
        trigger: "border-border text-primary border bg-transparent",
        portal: "border-border text-primary bg-background border",
      },
    },
  },
  defaultVariants: {
    variant: "primary",
  },
});

type Variants = VariantProps<typeof select>;

interface SelectProps extends Variants {
  options?: string[];
  placeholder?: string;
  ariaLabel?: string;
  value?: string | null;
  loading?: boolean;
  label?: string;
  disallowEmptySelection?: boolean;
  allowDuplicateSelectionEvents?: boolean;
  onChange?: (value: string | null) => void;
}

const Select = (props: SelectProps) => {
  return (
    <KSelect
      options={props.options || []}
      placeholder={
        <div class="flex items-center gap-8">
          <Show when={props.loading}>
            <LoaderCircle class="size-16 animate-spin" />
          </Show>
          {props.placeholder}
        </div>
      }
      value={props.value}
      placement="bottom-start"
      gutter={8}
      disallowEmptySelection={props.disallowEmptySelection}
      allowDuplicateSelectionEvents={props.allowDuplicateSelectionEvents}
      onChange={props.onChange}
      itemComponent={(props) => (
        <KSelect.Item
          item={props.item}
          class="focus:bg-highlight focus:outline-hidden flex h-32 cursor-pointer select-none items-center gap-8 rounded-md px-8"
        >
          <KSelect.ItemLabel>{props.item.rawValue}</KSelect.ItemLabel>
          <KSelect.ItemIndicator class="ml-auto">
            <Check class="size-16" />
          </KSelect.ItemIndicator>
        </KSelect.Item>
      )}
    >
      <div class="flex flex-col gap-8">
        <Show when={props.label}>
          <KSelect.Label class="text-secondary text-base font-light">
            {props.label}
          </KSelect.Label>
        </Show>
        <KSelect.Trigger
          aria-label={props.ariaLabel}
          class={select({ variant: props.variant }).trigger()}
        >
          <KSelect.Value<string> class="grow overflow-hidden text-ellipsis whitespace-nowrap text-left">
            {(state) => state.selectedOption()}
          </KSelect.Value>
          <KSelect.Icon class="ml-auto text-wrap">
            <ChevronsUpDown class="size-16" />
          </KSelect.Icon>
        </KSelect.Trigger>
      </div>
      <KSelect.Portal>
        <KSelect.Content class="select__content z-50">
          <KSelect.Listbox
            class={select({ variant: props.variant }).portal()}
          />
        </KSelect.Content>
      </KSelect.Portal>
    </KSelect>
  );
};

export default Select; */
import type { JSXElement, ValidComponent } from "solid-js";
import { splitProps } from "solid-js";
import type { PolymorphicProps } from "@kobalte/core/polymorphic";
import * as SelectPrimitive from "@kobalte/core/select";
import { cn } from "../utils";
import { ChevronsUpDown } from "lucide-solid";

const Select = SelectPrimitive.Root;
const SelectValue = SelectPrimitive.Value;
const SelectHiddenSelect = SelectPrimitive.HiddenSelect;

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
        "border-border border-1 flex h-40 w-full items-center gap-8 rounded-md px-16 disabled:cursor-not-allowed disabled:opacity-50",
        local.class,
      )}
      {...others}
    >
      {local.children}
      <SelectPrimitive.Icon
        as={ChevronsUpDown}
        class="border-border ml-auto h-16 w-auto"
      />
    </SelectPrimitive.Trigger>
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
          "bg-popover text-popover-foreground animate-in fade-in-80 relative z-50 min-w-32 overflow-hidden rounded-md border shadow-md",
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
        "focus:bg-accent focus:text-accent-foreground relative mt-0 flex w-full cursor-default select-none items-center rounded-sm py-1.5 pl-2 pr-8 text-sm outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
        local.class,
      )}
      {...others}
    >
      <SelectPrimitive.ItemIndicator class="absolute right-2 flex size-3.5 items-center justify-center">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="size-4"
        >
          <path stroke="none" d="M0 0h24v24H0z" fill="none" />
          <path d="M5 12l5 5l10 -10" />
        </svg>
      </SelectPrimitive.ItemIndicator>
      <SelectPrimitive.ItemLabel>{local.children}</SelectPrimitive.ItemLabel>
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
};

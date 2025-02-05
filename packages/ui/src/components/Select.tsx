import { Select as KSelect } from "@kobalte/core/select";
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
          class="focus:bg-highlighted focus:outline-hidden flex h-32 cursor-pointer select-none items-center gap-8 rounded-md px-8"
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

export default Select;

import { Select as KSelect } from "@kobalte/core/select";
import { Check, ChevronsUpDown, LoaderCircle } from "lucide-solid";
import type { VariantProps } from "tailwind-variants";
import { tv } from "tailwind-variants";
import "./styles.css";
import { Show } from "solid-js";

const triggerVariants = tv({
  base: "min-w-136 flex h-40 items-center gap-8 rounded px-16",
  variants: {
    variant: {
      primary: "bg-primary",
      secondary: "bg-secondary",
      outline: "border-border text-primary border bg-transparent",
    },
  },
  defaultVariants: {
    variant: "primary",
  },
});

const portalVariants = tv({
  base: "overflow-hidden rounded p-8",
  variants: {
    variant: {
      primary: "bg-primary",
      secondary: "bg-secondary",
      outline: "border-border text-primary bg-background border",
    },
  },
  defaultVariants: {
    variant: "primary",
  },
});

type SelectVariants = VariantProps<typeof triggerVariants>;
type PortalVariants = VariantProps<typeof portalVariants>;

interface SelectProps extends SelectVariants, PortalVariants {
  options?: string[];
  placeholder?: string;
  ariaLabel?: string;
  value?: string | null;
  loading?: boolean;
  label?: string;
  disallowEmptySelection?: boolean;
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
      onChange={props.onChange}
      itemComponent={(props) => (
        <KSelect.Item
          item={props.item}
          class="focus:bg-accent flex h-32 items-center gap-8 rounded px-8 focus:outline-none"
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
          <KSelect.Label class="text-secondary font-light">
            {props.label}
          </KSelect.Label>
        </Show>
        <KSelect.Trigger
          aria-label={props.ariaLabel}
          class={triggerVariants({ variant: props.variant })}
        >
          <KSelect.Value<string> class="flex-grow overflow-hidden text-ellipsis whitespace-nowrap text-left">
            {(state) => state.selectedOption()}
          </KSelect.Value>
          <KSelect.Icon class="ml-auto text-wrap">
            <ChevronsUpDown class="size-16" />
          </KSelect.Icon>
        </KSelect.Trigger>
      </div>
      <KSelect.Portal>
        <KSelect.Content class="select__content z-50">
          <KSelect.Listbox class={portalVariants({ variant: props.variant })} />
        </KSelect.Content>
      </KSelect.Portal>
    </KSelect>
  );
};

export default Select;

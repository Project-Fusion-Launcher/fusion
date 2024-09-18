import { Select as KSelect } from "@kobalte/core/select";
import { Check, ChevronsUpDown } from "lucide-solid";
import type { VariantProps } from "tailwind-variants";
import { tv } from "tailwind-variants";
import "./styles.css";

const triggerVariants = tv({
  base: "w-192 flex h-32 items-center gap-8 rounded px-8",
  variants: {
    variant: {
      default: "bg-secondary",
      outline: "border-border border bg-transparent",
    },
  },
});

const portalVariants = tv({
  base: "overflow-hidden rounded",
  variants: {
    variant: {
      default: "bg-secondary",
      outline: "border-border border bg-transparent",
    },
  },
  defaultVariants: {
    variant: "default",
  },
});

type SelectVariants = VariantProps<typeof triggerVariants>;
type PortalVariants = VariantProps<typeof portalVariants>;

interface SelectProps extends SelectVariants, PortalVariants {
  options?: string[];
  placeholder?: string;
  ariaLabel?: string;
  value?: string;
  onChange?: (value: string | null) => void;
}

const Select = (props: SelectProps) => {
  return (
    <KSelect
      options={props.options || []}
      placeholder={props.placeholder}
      value={props.value}
      onChange={props.onChange}
      itemComponent={(props) => (
        <KSelect.Item
          item={props.item}
          class="hover:bg-accent flex h-32 items-center gap-8 px-8"
        >
          <KSelect.ItemLabel>{props.item.rawValue}</KSelect.ItemLabel>
          <KSelect.ItemIndicator class="ml-auto">
            <Check class="size-16" />
          </KSelect.ItemIndicator>
        </KSelect.Item>
      )}
    >
      <KSelect.Trigger
        aria-label={props.ariaLabel}
        class={triggerVariants({ variant: props.variant })}
      >
        <KSelect.Value<string> class="flex-grow overflow-hidden text-ellipsis text-left">
          {(state) => state.selectedOption()}
        </KSelect.Value>
        <KSelect.Icon class="ml-auto">
          <ChevronsUpDown class="size-16" />
        </KSelect.Icon>
      </KSelect.Trigger>
      <KSelect.Portal>
        <KSelect.Content class="select__content">
          <KSelect.Listbox class={portalVariants({ variant: props.variant })} />
        </KSelect.Content>
      </KSelect.Portal>
    </KSelect>
  );
};

export default Select;

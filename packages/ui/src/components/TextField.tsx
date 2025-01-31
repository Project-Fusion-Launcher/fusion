import { TextField as KTextField } from "@kobalte/core/text-field";
import { X } from "lucide-solid";
import type { Component } from "solid-js";
import { Show } from "solid-js";
import { Dynamic } from "solid-js/web";
import type { VariantProps } from "tailwind-variants";
import { tv } from "tailwind-variants";

const textField = tv({
  slots: {
    base: "flex w-full flex-col gap-8 transition-all",
    field:
      "focus-within:ring-accent flex cursor-text items-center gap-8 rounded transition-all focus-within:ring-2",
    icon: "stroke-primary shrink-0",
  },
  variants: {
    variant: {
      primary: {
        field: "bg-primary",
      },
      secondary: {
        field: "bg-secondary",
      },
      outline: {
        field: "border-border border bg-transparent",
      },
    },
    size: {
      sm: {
        field: "h-32 gap-8 px-8",
        icon: "size-16",
      },
      md: {
        field: "h-40 gap-12 px-16",
        icon: "size-16",
      },
      lg: {
        field: "h-48 gap-16 px-16",
        icon: "size-20",
      },
    },
    width: {
      half: {
        base: "w-1/2",
      },
      full: {
        base: "w-full flex-grow",
      },
    },
  },
  defaultVariants: {
    variant: "primary",
    size: "md",
    width: "full",
  },
});

type Variants = VariantProps<typeof textField>;

export interface TextFieldProps extends Variants {
  placeholder?: string;
  value: string;
  onInput?: (value: string) => void;
  onChange?: (value: string) => void;
  icon?: Component<{ class?: string }>;
  autocomplete?: string;
  label?: string;
}

const TextField = (props: TextFieldProps) => {
  let inputRef!: HTMLInputElement;
  let clearButtonRef!: SVGSVGElement;

  const handleClick = (e: MouseEvent) => {
    if (!clearButtonRef || !clearButtonRef.contains(e.target as Node)) {
      inputRef?.focus();
    } else {
      props.onChange?.("");
      props.onInput?.("");
    }
  };

  return (
    <KTextField
      onClick={handleClick}
      class={textField({ width: props.width }).base()}
    >
      <Show when={props.label}>
        <KTextField.Label class="text-secondary text-base font-light">
          {props.label}
        </KTextField.Label>
      </Show>
      <div
        class={textField({
          variant: props.variant,
          size: props.size,
        }).field()}
      >
        <Dynamic
          component={props.icon}
          class={textField({ size: props.size }).icon()}
        />
        <KTextField.Input
          spellcheck={false}
          value={props.value}
          onInput={(e) => props.onInput?.(e.currentTarget.value)}
          onChange={(e: { currentTarget: { value: string } }) =>
            props.onChange?.(e.currentTarget.value)
          }
          autocomplete={props.autocomplete}
          placeholder={props.placeholder}
          class="placeholder-secondary text-primary h-full w-full rounded bg-transparent focus:outline-none"
          ref={(el: HTMLInputElement) => {
            inputRef = el;
          }}
        />
        <X
          ref={(el) => {
            clearButtonRef = el;
          }}
          class={textField({ size: props.size }).icon() + " cursor-pointer"}
          classList={{ invisible: !props.value }}
        />
      </div>
    </KTextField>
  );
};

export default TextField;

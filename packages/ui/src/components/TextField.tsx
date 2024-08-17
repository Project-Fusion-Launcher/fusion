import { TextField as KTextField } from "@kobalte/core/text-field";
import { X } from "lucide-solid";
import type { Component } from "solid-js";
import { Show } from "solid-js";
import { Dynamic } from "solid-js/web";
import type { VariantProps } from "tailwind-variants";
import { tv } from "tailwind-variants";

const variants = tv({
  base: "focus-within:ring-accent flex items-center rounded focus-within:ring-2",
  variants: {
    variant: {
      default: "bg-border",
      outline: "border-border border bg-transparent",
    },
    size: {
      default: "h-32 gap-8 px-8",
      large: "h-48 gap-16 px-16",
    },
    width: {
      full: "w-full flex-grow",
    },
    defaultVariants: {
      variant: "default",
      size: "default",
    },
  },
});

const iconVariants = tv({
  base: "stroke-primary shrink-0",
  variants: {
    size: {
      default: "size-16",
      large: "size-20",
    },
  },
});

type TextFieldVariants = VariantProps<typeof variants>;

export interface TextFieldProps extends TextFieldVariants {
  placeholder?: string;
  value: string;
  onInput?: (value: string) => void;
  onChange?: (value: string) => void;
  icon?: Component;
  autocomplete?: string;
}

const TextField = (props: TextFieldProps) => {
  let inputRef!: HTMLInputElement;
  let clearButtonRef!: SVGSVGElement;

  const handleClick = (e: MouseEvent) => {
    console.log(e.target);
    if (!clearButtonRef.contains(e.target as Node)) {
      inputRef?.focus();
    } else {
      props.onChange?.("");
      props.onInput?.("");
    }
  };

  return (
    <KTextField
      class={variants({
        width: props.width,
        variant: props.variant,
        size: props.size,
      })}
      onClick={handleClick}
    >
      <Dynamic
        component={props.icon}
        class={iconVariants({ size: props.size })}
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
      <Show when={props.value}>
        <X
          ref={(el) => {
            clearButtonRef = el;
          }}
          class={iconVariants({ size: props.size }) + " cursor-pointer"}
        />
      </Show>
    </KTextField>
  );
};

export default TextField;

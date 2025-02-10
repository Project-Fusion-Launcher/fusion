/*import { TextField as KTextField } from "@kobalte/core/text-field";
import { X } from "lucide-solid";
import type { Component } from "solid-js";
import { Show } from "solid-js";
import { Dynamic } from "solid-js/web";
import type { VariantProps } from "tailwind-variants";
import { tv } from "tailwind-variants";

const textField = tv({
  slots: {
    base: "flex w-full flex-col gap-12 transition-all",
    field:
      "focus-within:ring-accent flex cursor-text items-center gap-8 rounded-md transition-all focus-within:ring-2",
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
        base: "w-full grow",
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
          class="placeholder-secondary text-primary focus:outline-hidden h-full w-full rounded-md bg-transparent placeholder:font-light"
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

export default TextField;*/

import { mergeProps, splitProps, type ValidComponent } from "solid-js";
import * as TextFieldPrimitive from "@kobalte/core/text-field";
import type { PolymorphicProps } from "@kobalte/core";
import { cn } from "../utils";
import type { VariantProps } from "tailwind-variants";
import { tv } from "tailwind-variants";

type TextFieldRootProps<T extends ValidComponent = "div"> =
  TextFieldPrimitive.TextFieldRootProps<T> & {
    class?: string | undefined;
  };

const TextField = <T extends ValidComponent = "div">(
  props: PolymorphicProps<T, TextFieldRootProps<T>>,
) => {
  const [local, others] = splitProps(props as TextFieldRootProps, ["class"]);
  return (
    <TextFieldPrimitive.Root
      class={cn("flex flex-col gap-12", local.class)}
      {...others}
    />
  );
};

const textFieldInputVariants = tv({
  base: "focus-within:ring-accent border-radi border-border border-1 flex w-full cursor-text items-center gap-8 rounded-md transition-all placeholder:font-light focus-within:ring-2 focus:outline-none",
  variants: {
    size: {
      md: "h-40 gap-12 px-16 text-sm",
      lg: "h-48 gap-16 px-16",
    },
  },
  defaultVariants: {
    size: "md",
  },
});

type TextFieldInputProps<T extends ValidComponent = "input"> =
  TextFieldPrimitive.TextFieldInputProps<T> &
    VariantProps<typeof textFieldInputVariants> & {
      class?: string | undefined;
      type?:
        | "button"
        | "checkbox"
        | "color"
        | "date"
        | "datetime-local"
        | "email"
        | "file"
        | "hidden"
        | "image"
        | "month"
        | "number"
        | "password"
        | "radio"
        | "range"
        | "reset"
        | "search"
        | "submit"
        | "tel"
        | "text"
        | "time"
        | "url"
        | "week";
    };

const TextFieldInput = <T extends ValidComponent = "input">(
  rawProps: PolymorphicProps<T, TextFieldInputProps<T>>,
) => {
  const props = mergeProps<TextFieldInputProps<T>[]>(
    { type: "text" },
    rawProps,
  );
  const [local, others] = splitProps(props as TextFieldInputProps, [
    "type",
    "size",
    "class",
  ]);
  return (
    <div class="relative flex items-center">
      <TextFieldPrimitive.Input
        type={local.type}
        class={cn(textFieldInputVariants({ size: local.size }), local.class)}
        {...others}
      />
    </div>
  );
};

type TextFieldLabelProps<T extends ValidComponent = "label"> =
  TextFieldPrimitive.TextFieldLabelProps<T> & { class?: string | undefined };

const TextFieldLabel = <T extends ValidComponent = "label">(
  props: PolymorphicProps<T, TextFieldLabelProps<T>>,
) => {
  const [local, others] = splitProps(props as TextFieldLabelProps, ["class"]);
  return (
    <TextFieldPrimitive.Label
      class={cn("text-secondary text-base font-light", local.class)}
      {...others}
    />
  );
};

export { TextField, TextFieldInput, TextFieldLabel };

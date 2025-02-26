import type { Component } from "solid-js";
import { mergeProps, Show, splitProps, type ValidComponent } from "solid-js";
import * as TextFieldPrimitive from "@kobalte/core/text-field";
import type { PolymorphicProps } from "@kobalte/core";
import { cn } from "../utils";
import type { VariantProps } from "tailwind-variants";
import { tv } from "tailwind-variants";
import { Dynamic } from "solid-js/web";

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
  base: "focus-within:ring-accent border-border border-1 flex w-full cursor-text items-center gap-8 rounded-md transition-all placeholder:font-light focus-within:ring-2 focus:outline-none",
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
      icon?: Component<{
        class?: string;
      }>;
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
    "icon",
  ]);
  return (
    <div class="flex w-full items-center">
      <Show when={local.icon}>
        <Dynamic
          component={local.icon}
          class="text-secondary pointer-events-none absolute ml-16 size-16"
        />
      </Show>
      <TextFieldPrimitive.Input
        type={local.type}
        class={cn(
          textFieldInputVariants({ size: local.size }),
          local.class,
          local.icon && "pl-48",
        )}
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

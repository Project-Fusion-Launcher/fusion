import type { JSXElement, ValidComponent } from "solid-js";
import { splitProps } from "solid-js";
import type { VariantProps } from "tailwind-variants";
import { tv } from "tailwind-variants";
import * as ButtonPrimitive from "@kobalte/core/button";
import type { PolymorphicProps } from "@kobalte/core";
import { cn } from "../utils";

const buttonVariants = tv({
  base: "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium disabled:pointer-events-none disabled:opacity-50",
  variants: {
    variant: {
      primary: "bg-primary text-primary-foreground",
      secondary: "bg-secondary text-secondary-foreground",
      accent: "bg-accent text-accent-foreground",
      ghost: "bg-transparent",
      outline: "border-border text-primary border",
    },
    size: {
      sm: "h-32 px-32 text-xs",
      md: "h-40 px-40",
    },
  },
  compoundVariants: [{ variant: "ghost", class: "px-0" }],
  defaultVariants: {
    variant: "primary",
    size: "md",
  },
});

type ButtonProps<T extends ValidComponent = "button"> =
  ButtonPrimitive.ButtonRootProps<T> &
    VariantProps<typeof buttonVariants> & {
      class?: string | undefined;
      children?: JSXElement;
    };

const Button = <T extends ValidComponent = "button">(
  props: PolymorphicProps<T, ButtonProps<T>>,
) => {
  const [local, others] = splitProps(props as ButtonProps, [
    "variant",
    "size",
    "class",
  ]);
  return (
    <ButtonPrimitive.Root
      class={cn(
        buttonVariants({ variant: local.variant, size: local.size }),
        local.class,
      )}
      {...others}
    />
  );
};

export default Button;

import type { JSXElement, ValidComponent } from "solid-js";
import { splitProps } from "solid-js";
import type { VariantProps } from "tailwind-variants";
import { tv } from "tailwind-variants";
import * as ButtonPrimitive from "@kobalte/core/button";
import type { PolymorphicProps } from "@kobalte/core";
import { cn } from "../utils";

const iconButtonVariants = tv({
  base: "inline-flex shrink-0 items-center justify-center whitespace-nowrap rounded-md text-sm font-medium disabled:pointer-events-none disabled:opacity-50",
  variants: {
    variant: {
      primary: "bg-primary text-primary-foreground",
      secondary: "bg-secondary text-secondary-foreground",
      accent: "bg-accent text-accent-foreground",
      ghost: "bg-transparent",
      outline: "border-border text-primary border",
    },
    size: {
      sm: "size-32 shrink-0 [&>*]:size-12",
      md: "size-40 shrink-0 [&>*]:size-16",
      lg: "size-48 shrink-0 [&>*]:size-20",
    },
  },
  compoundVariants: [{ variant: "ghost", class: "px-0" }],
  defaultVariants: {
    variant: "primary",
    size: "md",
  },
});

type IconButtonProps<T extends ValidComponent = "button"> =
  ButtonPrimitive.ButtonRootProps<T> &
    VariantProps<typeof iconButtonVariants> & {
      class?: string | undefined;
      children?: JSXElement;
      circle?: boolean;
    };

const IconButton = <T extends ValidComponent = "button">(
  props: PolymorphicProps<T, IconButtonProps<T>>,
) => {
  const [local, others] = splitProps(props as IconButtonProps, [
    "variant",
    "size",
    "class",
    "circle",
  ]);
  return (
    <ButtonPrimitive.Root
      class={cn(
        iconButtonVariants({ variant: local.variant, size: local.size }),
        local.class,
        local.circle && "rounded-full",
      )}
      {...others}
    />
  );
};

export default IconButton;

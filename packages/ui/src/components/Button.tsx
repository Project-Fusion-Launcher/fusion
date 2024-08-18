import { Button as KButton } from "@kobalte/core/button";
import type { JSX } from "solid-js";
import type { VariantProps } from "tailwind-variants";
import { tv } from "tailwind-variants";

const variants = tv({
  base: "flex items-center justify-center gap-8 rounded px-16 py-8",
  variants: {
    variant: {
      primary: "bg-primary text-bg",
      secondary: "bg-secondary text-bg",
      accent: "bg-accent text-primary",
      ghost: "bg-transparen p-0",
      outline: "border-border border",
    },
    size: {
      md: "h-32",
    },
  },
  defaultVariants: {
    variant: "primary",
    size: "md",
  },
});

type ButtonVariants = VariantProps<typeof variants>;

export interface ButtonProps extends ButtonVariants {
  children: JSX.Element;
}

const Button = (props: ButtonProps) => {
  return (
    <KButton class={variants({ variant: props.variant })}>
      {props.children}
    </KButton>
  );
};

export default Button;

import { Button as KButton } from "@kobalte/core/button";
import type { JSX } from "solid-js";
import type { VariantProps } from "tailwind-variants";
import { tv } from "tailwind-variants";

const variants = tv({
  base: "flex items-center justify-center gap-8 rounded",
  variants: {
    variant: {
      primary: "bg-primary text-bg",
      secondary: "bg-secondary text-bg",
      accent: "bg-accent text-primary",
      ghost: "bg-transparen",
      outline: "border-border text-primary border",
    },
    disabled: {
      true: "disabled cursor-not-allowed opacity-50",
    },
    size: {
      sm: "h-32 px-32 text-sm",
      md: "h-40 px-40",
    },
  },
  compoundVariants: [{ variant: "ghost", class: "px-0" }],
  defaultVariants: {
    variant: "primary",
    size: "md",
  },
});

type ButtonVariants = VariantProps<typeof variants>;

export interface ButtonProps extends ButtonVariants {
  children: JSX.Element;
  onClick?: () => void;
}

const Button = (props: ButtonProps) => {
  return (
    <KButton
      class={variants({
        variant: props.variant,
        size: props.size,
        disabled: props.disabled,
      })}
      onClick={props.onClick}
    >
      {props.children}
    </KButton>
  );
};

export default Button;

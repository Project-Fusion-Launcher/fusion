import { Button as KButton } from "@kobalte/core/button";
import type { JSXElement } from "solid-js";
import type { VariantProps } from "tailwind-variants";
import { tv } from "tailwind-variants";

const variants = tv({
  base: "flex items-center justify-center",
  variants: {
    variant: {
      primary: "bg-primary text-bg",
      secondary: "bg-secondary text-bg",
      accent: "bg-accent text-primary",
      ghost: "bg-transparent",
      outline: "border-border text-primary border",
    },
    disabled: {
      true: "disabled cursor-not-allowed opacity-50",
    },
    size: {
      sm: "size-32 flex-shrink-0 [&>*]:size-12",
      md: "size-40 flex-shrink-0 [&>*]:size-16",
      lg: "size-48 flex-shrink-0 [&>*]:size-20",
    },
    shape: {
      circle: "rounded-full",
      square: "rounded",
    },
  },
  compoundVariants: [{ variant: "ghost", class: "px-0" }],
  defaultVariants: {
    variant: "primary",
    size: "md",
    shape: "square",
  },
});

type IconButtonVariants = VariantProps<typeof variants>;

export interface IconButtonProps extends IconButtonVariants {
  children: JSXElement;
  onClick?: () => void;
}

const IconButton = (props: IconButtonProps) => {
  return (
    <KButton
      class={variants({
        variant: props.variant,
        size: props.size,
        disabled: props.disabled,
        shape: props.shape,
      })}
      onClick={props.onClick}
    >
      {props.children}
    </KButton>
  );
};

export default IconButton;

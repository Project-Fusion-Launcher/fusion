import { Button as KButton } from "@kobalte/core/button";
import { LoaderCircle } from "lucide-solid";
import type { JSXElement } from "solid-js";
import { Show } from "solid-js";
import type { VariantProps } from "tailwind-variants";
import { tv } from "tailwind-variants";

const variants = tv({
  base: "relative flex items-center justify-center rounded-md",
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
  children: JSXElement;
  onClick?: () => void;
  loading?: boolean;
}

const Button = (props: ButtonProps) => {
  const handleClick = () => {
    if (!props.disabled && props.onClick) {
      props.onClick();
    }
  };

  return (
    <KButton
      class={variants({
        variant: props.variant,
        size: props.size,
        disabled: props.disabled,
      })}
      onClick={handleClick}
    >
      <Show when={props.loading}>
        <LoaderCircle class="absolute size-16 animate-spin" />
      </Show>
      <div
        class="flex items-center justify-center gap-8"
        classList={{ "opacity-0": props.loading }}
      >
        {props.children}
      </div>
    </KButton>
  );
};

export default Button;

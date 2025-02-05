import type { Component, ComponentProps } from "solid-js";
import { splitProps } from "solid-js";
import { cn } from "../utils";
import type { VariantProps } from "tailwind-variants";
import { tv } from "tailwind-variants";

const badgeVariants = tv({
  base: "flex w-min items-center justify-center rounded border font-light font-medium transition-colors",
  variants: {
    variant: {
      primary: "bg-primary text-background border-transparent",
      secondary: "bg-secondary text-background border-transparent",
      outline: "border-border text-secondary",
      accent: "bg-accent text-primary border-transparent",
    },
    size: {
      sm: "text-xm h-24 px-[6px]",
      md: "h-28 px-8",
    },
  },
  defaultVariants: {
    variant: "primary",
    size: "md",
  },
});

type BadgeProps = ComponentProps<"div"> & VariantProps<typeof badgeVariants>;

const Badge: Component<BadgeProps> = (props) => {
  const [local, others] = splitProps(props, ["class", "variant", "size"]);
  return (
    <div
      class={cn(
        badgeVariants({
          variant: local.variant,
          size: local.size,
        }),
        local.class,
      )}
      {...others}
    />
  );
};

export default Badge;

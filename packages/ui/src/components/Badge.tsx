import type { Component, ComponentProps } from "solid-js";
import { splitProps } from "solid-js";
import { cn } from "../utils";
import type { VariantProps } from "tailwind-variants";
import { tv } from "tailwind-variants";

const badgeVariants = tv({
  base: "flex w-min items-center justify-center rounded-md border border-transparent font-medium transition-colors",
  variants: {
    variant: {
      primary: "bg-primary text-primary-foreground",
      secondary: "bg-secondary text-secondary-foreground",
      outline: "border-border text-secondary",
      accent: "bg-accent text-primary",
    },
    size: {
      sm: "h-24 px-[6px] text-xs",
      md: "h-28 px-8 text-sm",
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

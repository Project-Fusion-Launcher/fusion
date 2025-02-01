import type { VariantProps } from "tailwind-variants";
import { tv } from "tailwind-variants";

const variants = tv({
  base: "flex w-min items-center justify-center rounded font-light transition-colors",
  variants: {
    variant: {
      primary: "bg-primary text-bg",
      secondary: "bg-secondary text-bg",
      accent: "bg-accent text-primary",
      outline: "outline-border text-secondary outline outline-1",
    },
    size: {
      sm: "h-24 px-[6px] text-sm",
      md: "h-28 px-8",
    },
  },
  defaultVariants: {
    variant: "primary",
    size: "md",
  },
});

type BadgeVariants = VariantProps<typeof variants>;

export interface BadgeProps extends BadgeVariants {
  children?: string | number;
}

const Badge = (props: BadgeProps) => {
  return (
    <div class={variants({ variant: props.variant, size: props.size })}>
      {props.children}
    </div>
  );
};

export default Badge;

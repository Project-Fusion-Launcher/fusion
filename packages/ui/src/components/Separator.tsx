import { Separator as KSeparator } from "@kobalte/core/separator";
import type { VariantProps } from "tailwind-variants";
import { tv } from "tailwind-variants";

const variants = tv({
  base: "bg-border m-8 h-1 w-32 border-0",
  variants: {
    width: {
      default: "w-32",
      full: "w-full flex-grow",
    },
  },
  defaultVariants: {
    width: "default",
  },
});

type SeparatorProps = VariantProps<typeof variants>;

const Separator = (props: SeparatorProps) => {
  return <KSeparator class={variants({ width: props.width })} />;
};

export default Separator;

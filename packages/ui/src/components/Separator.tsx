import { Separator as KSeparator } from "@kobalte/core/separator";
import type { VariantProps } from "tailwind-variants";
import { tv } from "tailwind-variants";

const variants = tv({
  base: "bg-border m-8 h-1 border-none",
  variants: {
    width: {
      half: "w-1/2",
      "75": "w-3/4",
      full: "w-full grow",
    },
  },
  defaultVariants: {
    width: "half",
  },
});

type SeparatorProps = VariantProps<typeof variants>;

const Separator = (props: SeparatorProps) => {
  return <KSeparator class={variants({ width: props.width })} />;
};

export default Separator;

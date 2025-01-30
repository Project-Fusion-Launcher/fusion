import { Tooltip as KTooltip } from "@kobalte/core/tooltip";
import type { JSXElement } from "solid-js";
import type { VariantProps } from "tailwind-variants";
import { tv } from "tailwind-variants";

const variants = tv({
  base: "fixed z-50 flex h-40 items-center gap-12 rounded px-16",
  variants: {
    variant: {
      outline: "border-border text-primary bg-background border",
    },
  },
  defaultVariants: {
    variant: "outline",
  },
});

type Variants = VariantProps<typeof variants>;

interface TooltipProps extends Variants {
  children: JSXElement;
  content: string;
  class?: string;
  as?: string;
}

const Tooltip = (props: TooltipProps) => {
  return (
    <KTooltip openDelay={200} overflowPadding={40}>
      <KTooltip.Trigger class={props.class} as={props.as}>
        {props.children}
      </KTooltip.Trigger>
      <KTooltip.Portal>
        <KTooltip.Content
          class={"tooltip__content " + variants({ variant: props.variant })}
        >
          <KTooltip.Arrow />
          {props.content}
        </KTooltip.Content>
      </KTooltip.Portal>
    </KTooltip>
  );
};

export default Tooltip;

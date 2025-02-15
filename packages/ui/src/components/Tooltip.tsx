import type { ValidComponent } from "solid-js";
import { splitProps, type Component } from "solid-js";
import type { PolymorphicProps } from "@kobalte/core/polymorphic";
import * as TooltipPrimitive from "@kobalte/core/tooltip";
import { cn } from "../utils";

const TooltipTrigger = TooltipPrimitive.Trigger;

const Tooltip: Component<TooltipPrimitive.TooltipRootProps> = (props) => {
  return (
    <TooltipPrimitive.Root
      openDelay={300}
      overflowPadding={40}
      gutter={8}
      {...props}
    />
  );
};

type TooltipContentProps<T extends ValidComponent = "div"> =
  TooltipPrimitive.TooltipContentProps<T> & { class?: string | undefined };

const TooltipContent = <T extends ValidComponent = "div">(
  props: PolymorphicProps<T, TooltipContentProps<T>>,
) => {
  const [local, others] = splitProps(props as TooltipContentProps, ["class"]);
  return (
    <TooltipPrimitive.Portal>
      <TooltipPrimitive.Content
        class={cn(
          "bg-popover text-popover-foreground animate-pop-out border-border data-[expanded]:animate-pop-in z-50 inline-flex origin-[var(--kb-popover-content-transform-origin)] items-center justify-center overflow-hidden rounded-md border p-8 text-sm font-medium shadow-md",
          local.class,
        )}
        {...others}
      />
    </TooltipPrimitive.Portal>
  );
};

export { Tooltip, TooltipTrigger, TooltipContent };

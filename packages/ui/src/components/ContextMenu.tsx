import type { PolymorphicProps } from "@kobalte/core";
import * as ContextMenuPrimitive from "@kobalte/core/context-menu";
import type { JSXElement } from "solid-js";
import { splitProps, type Component, type ValidComponent } from "solid-js";
import { cn } from "../utils";
import type { VariantProps } from "tailwind-variants";
import { tv } from "tailwind-variants";
import { ChevronRight } from "lucide-solid";

const ContextMenuTrigger = ContextMenuPrimitive.Trigger;
const ContextMenuSub = ContextMenuPrimitive.Sub;

const ContextMenu: Component<ContextMenuPrimitive.ContextMenuRootProps> = (
  props,
) => {
  return <ContextMenuPrimitive.Root gutter={4} {...props} />;
};

type ContextMenuContentProps<T extends ValidComponent = "div"> =
  ContextMenuPrimitive.ContextMenuContentProps<T> & {
    class?: string;
  };

const ContextMenuContent = <T extends ValidComponent = "div">(
  props: PolymorphicProps<T, ContextMenuContentProps<T>>,
) => {
  const [local, others] = splitProps(props as ContextMenuContentProps, [
    "class",
  ]);
  return (
    <ContextMenuPrimitive.Portal>
      <ContextMenuPrimitive.Content
        class={cn(
          "border-border text-popover-foreground bg-popover context-menu__content data-[expanded]:animate-pop-in origin-[var(--kb-menu-content-transform-origin)] overflow-hidden rounded-md border p-4 text-sm",
          local.class,
        )}
        {...others}
      />
    </ContextMenuPrimitive.Portal>
  );
};

const contextMenuItemVariants = tv({
  base: "focus:outline-hidden relative flex h-32 cursor-pointer select-none items-center gap-8 rounded-sm px-8 data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
  variants: {
    variant: {
      default: "focus:bg-highlight text-primary",
      primary: "bg-primary text-primary-foreground",
      accent: "bg-accent text-primary",
      destructive: "focus:bg-highlight text-danger",
    },
  },
  defaultVariants: {
    variant: "default",
  },
});

type ContextMenuItemProps<T extends ValidComponent = "div"> =
  ContextMenuPrimitive.ContextMenuItemProps<T> &
    VariantProps<typeof contextMenuItemVariants> & {
      class?: string | undefined;
    };

const ContextMenuItem = <T extends ValidComponent = "div">(
  props: PolymorphicProps<T, ContextMenuItemProps<T>>,
) => {
  const [local, others] = splitProps(props as ContextMenuItemProps, [
    "class",
    "variant",
  ]);
  return (
    <ContextMenuPrimitive.Item
      class={cn(
        contextMenuItemVariants({ variant: local.variant }),
        local.class,
      )}
      {...others}
    />
  );
};

type ContextMenuSeparatorProps<T extends ValidComponent = "hr"> =
  ContextMenuPrimitive.ContextMenuSeparatorProps<T> & {
    class?: string | undefined;
  };

const ContextMenuSeparator = <T extends ValidComponent = "hr">(
  props: PolymorphicProps<T, ContextMenuSeparatorProps<T>>,
) => {
  const [local, others] = splitProps(props as ContextMenuSeparatorProps, [
    "class",
  ]);
  return (
    <ContextMenuPrimitive.Separator
      class={cn("bg-border -mx-4 my-4 h-1 border-none", local.class)}
      {...others}
    />
  );
};

type ContextMenuSubTriggerProps<T extends ValidComponent = "div"> =
  ContextMenuPrimitive.ContextMenuSubTriggerProps<T> & {
    class?: string | undefined;
    children?: JSXElement;
  };

const ContextMenuSubTrigger = <T extends ValidComponent = "div">(
  props: PolymorphicProps<T, ContextMenuSubTriggerProps<T>>,
) => {
  const [local, others] = splitProps(props as ContextMenuSubTriggerProps, [
    "class",
    "children",
  ]);
  return (
    <ContextMenuPrimitive.SubTrigger
      class={cn(
        "data-[expanded]:bg-highlight hover:bg-highlight focus:outline-hidden relative flex h-32 cursor-pointer select-none items-center gap-8 rounded-sm px-8 data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
        local.class,
      )}
      {...others}
    >
      {local.children}
      <ChevronRight class="ml-auto size-16" />
    </ContextMenuPrimitive.SubTrigger>
  );
};

type ContextMenuSubContentProps<T extends ValidComponent = "div"> =
  ContextMenuPrimitive.ContextMenuSubContentProps<T> & {
    class?: string | undefined;
  };

const ContextMenuSubContent = <T extends ValidComponent = "div">(
  props: PolymorphicProps<T, ContextMenuSubContentProps<T>>,
) => {
  const [local, others] = splitProps(props as ContextMenuSubContentProps, [
    "class",
  ]);
  return (
    <ContextMenuPrimitive.Portal>
      <ContextMenuPrimitive.SubContent
        class={cn(
          "border-border text-popover-foreground bg-popover context-menu__content data-[expanded]:animate-pop-in origin-[var(--kb-menu-content-transform-origin)] overflow-hidden rounded-md border p-4 text-sm",
          local.class,
        )}
        {...others}
      />
    </ContextMenuPrimitive.Portal>
  );
};

export {
  ContextMenu,
  ContextMenuTrigger,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuSeparator,
  ContextMenuSub,
  ContextMenuSubTrigger,
  ContextMenuSubContent,
};

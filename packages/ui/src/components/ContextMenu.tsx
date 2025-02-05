import { ContextMenu as KContextMenu } from "@kobalte/core/context-menu";
import type { JSXElement } from "solid-js";
import type { VariantProps } from "tailwind-variants";
import { tv } from "tailwind-variants";

const itemVariants = tv({
  base: "focus:outline-hidden flex h-32 cursor-pointer select-none items-center gap-8 rounded px-8",
  variants: {
    variant: {
      default: "focus:bg-highlighted text-primary",
      accent: "bg-accent text-primary",
      primary: "bg-primary text-background",
      danger: "focus:bg-highlighted text-danger",
    },
  },
  defaultVariants: {
    variant: "default",
  },
});

type ItemVariants = VariantProps<typeof itemVariants>;

export interface ContextMenuItemProps extends ItemVariants {
  children: JSXElement;
  onSelect?: () => void;
}

const ContextMenuItem = (props: ContextMenuItemProps) => {
  return (
    <KContextMenu.Item
      class={itemVariants({ variant: props.variant })}
      onSelect={props.onSelect}
    >
      {props.children}
    </KContextMenu.Item>
  );
};

export interface ContextMenuSubTriggerProps extends ItemVariants {
  children: JSXElement;
}

const ContextMenuSubTrigger = (props: ContextMenuSubTriggerProps) => {
  return (
    <KContextMenu.SubTrigger class={itemVariants({ variant: props.variant })}>
      {props.children}
    </KContextMenu.SubTrigger>
  );
};

const contentVariants = tv({
  base: "border-border text-primary bg-background context-menu__content overflow-hidden rounded border p-8",
});

const ContextMenuContent = (props: { children: JSXElement }) => {
  return (
    <KContextMenu.Content class={contentVariants()}>
      {props.children}
    </KContextMenu.Content>
  );
};

const ContextMenuSubContent = (props: { children: JSXElement }) => {
  return (
    <KContextMenu.SubContent class={contentVariants()}>
      {props.children}
    </KContextMenu.SubContent>
  );
};

const ContextMenuSeparator = () => {
  return <KContextMenu.Separator class="bg-border m-8 h-1 border-none" />;
};

export {
  ContextMenuItem,
  ContextMenuSubTrigger,
  ContextMenuContent,
  ContextMenuSubContent,
  ContextMenuSeparator,
};

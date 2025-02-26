import type { PolymorphicProps } from "@kobalte/core";
import * as DialogPrimitive from "@kobalte/core/dialog";
import type {
  ValidComponent,
  Component,
  JSXElement,
  ComponentProps,
} from "solid-js";
import { splitProps } from "solid-js";
import { cn } from "../utils";
import { X } from "lucide-solid";

const Dialog = DialogPrimitive.Root;
const DialogTrigger = DialogPrimitive.Trigger;

const DialogPortal: Component<DialogPrimitive.DialogPortalProps> = (props) => {
  const [local, other] = splitProps(props, ["children"]);
  return (
    <DialogPrimitive.Portal {...other}>
      <div class="fixed inset-0 z-50 flex items-center justify-center">
        {local.children}
      </div>
    </DialogPrimitive.Portal>
  );
};

type DialogOverlayProps<T extends ValidComponent = "div"> =
  DialogPrimitive.DialogOverlayProps<T> & { class?: string | undefined };

const DialogOverlay = <T extends ValidComponent = "div">(
  props: PolymorphicProps<T, DialogOverlayProps<T>>,
) => {
  const [local, other] = splitProps(props as DialogOverlayProps, ["class"]);
  return (
    <DialogPrimitive.Overlay
      class={cn(
        "bg-background/80 data-[expanded]:animate-fade-in data-[closed]:animate-fade-out fixed inset-0 z-50",
        local.class,
      )}
      {...other}
    />
  );
};

type DialogContentProps<T extends ValidComponent = "div"> =
  DialogPrimitive.DialogContentProps<T> & {
    class?: string | undefined;
    children?: JSXElement;
  };

const DialogContent = <T extends ValidComponent = "div">(
  props: PolymorphicProps<T, DialogContentProps<T>>,
) => {
  const [local, other] = splitProps(props as DialogContentProps, [
    "class",
    "children",
  ]);
  return (
    <DialogPortal>
      <DialogOverlay />
      <DialogPrimitive.Content
        class={cn(
          "bg-background data-[expanded]:animate-in data-[closed]:animate-out border-border border-1 absolute z-50 grid overflow-y-auto rounded-lg p-32",
          local.class,
        )}
        {...other}
      >
        {local.children}
        <DialogPrimitive.CloseButton class="text-secondary absolute right-32 top-[30px] size-24">
          <X class="size-24" />
          <span class="sr-only">Close</span>
        </DialogPrimitive.CloseButton>
      </DialogPrimitive.Content>
    </DialogPortal>
  );
};

const DialogHeader: Component<ComponentProps<"div">> = (props) => {
  const [local, other] = splitProps(props, ["class"]);
  return (
    <div
      class={cn("mb-32 flex flex-col gap-8 text-left", local.class)}
      {...other}
    />
  );
};

const DialogFooter: Component<ComponentProps<"div">> = (props) => {
  const [local, other] = splitProps(props, ["class"]);
  return <div class={cn("flex flex-row-reverse", local.class)} {...other} />;
};

type DialogTitleProps<T extends ValidComponent = "h2"> =
  DialogPrimitive.DialogTitleProps<T> & {
    class?: string | undefined;
  };

const DialogTitle = <T extends ValidComponent = "h2">(
  props: PolymorphicProps<T, DialogTitleProps<T>>,
) => {
  const [, rest] = splitProps(props as DialogTitleProps, ["class"]);
  return (
    <DialogPrimitive.Title
      class={cn("text-lg font-bold", props.class)}
      {...rest}
    />
  );
};

type DialogDescriptionProps<T extends ValidComponent = "p"> =
  DialogPrimitive.DialogDescriptionProps<T> & {
    class?: string | undefined;
  };

const DialogDescription = <T extends ValidComponent = "p">(
  props: PolymorphicProps<T, DialogDescriptionProps<T>>,
) => {
  const [, rest] = splitProps(props as DialogDescriptionProps, ["class"]);
  return (
    <DialogPrimitive.Description
      class={cn("text-secondary text-base font-medium", props.class)}
      {...rest}
    />
  );
};

export {
  Dialog,
  DialogTrigger,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
  DialogDescription,
};

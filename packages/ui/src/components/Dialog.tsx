import { Dialog as KDialog } from "@kobalte/core/dialog";
import { X } from "lucide-solid";
import type { JSX } from "solid-js";

export interface DialogProps {
  title: string;
  children?: JSX.Element;
  defaultOpen?: boolean;
  open?: boolean;
  onOpenChange?: (isOpen: boolean) => void;
}

const Dialog = (props: DialogProps) => {
  return (
    <KDialog
      defaultOpen={props.defaultOpen}
      open={props.open}
      onOpenChange={props.onOpenChange}
    >
      <KDialog.Portal>
        <KDialog.Overlay class="dialog__overlay fixed inset-0 z-50 h-full w-full bg-black bg-opacity-55" />
        <div class="fixed inset-0 z-50 flex h-full w-full items-center justify-center">
          <KDialog.Content
            class="dialog__content min-h-288 bg-background border-border relative z-50 flex flex-col overflow-hidden rounded-lg border p-40 font-medium"
            style={{ "min-width": `calc(${props.title.length}ch + 12rem)` }}
          >
            <div class="text-primary mb-40 flex w-full items-center justify-center gap-48">
              <KDialog.Title class="h-min text-base font-bold">
                {props.title.toUpperCase()}
              </KDialog.Title>
              <KDialog.CloseButton class="text-secondary absolute right-40 ml-auto size-24">
                <X class="size-24" />
              </KDialog.CloseButton>
            </div>
            <div class="relative w-full flex-grow">{props.children}</div>
          </KDialog.Content>
        </div>
      </KDialog.Portal>
    </KDialog>
  );
};

export default Dialog;

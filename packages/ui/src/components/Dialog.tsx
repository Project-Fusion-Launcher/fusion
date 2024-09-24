import { Dialog as KDialog } from "@kobalte/core/dialog";
import { X } from "lucide-solid";
import type { JSX } from "solid-js";

export interface DialogProps {
  title: string;
  children?: JSX.Element;
  defaultOpen?: boolean;
  open?: boolean;
  onOpenChange?: (open: boolean) => void;
}

const Dialog = (props: DialogProps) => {
  return (
    <KDialog
      defaultOpen={props.defaultOpen}
      open={props.open}
      onOpenChange={props.onOpenChange}
    >
      <KDialog.Portal>
        <KDialog.Overlay class="bg-highlighted fixed inset-0 z-50 h-full w-full" />
        <div class="fixed inset-0 z-50 flex h-full w-full items-center justify-center">
          <KDialog.Content class="min-w-288 min-h-288 bg-background border-t-accent border-t-md relative z-50 flex flex-col overflow-hidden rounded p-24 font-medium">
            <div class="mb-24 flex w-full items-center">
              <KDialog.Title class="font-lightmedium h-min text-lg">
                {props.title}
              </KDialog.Title>
              <KDialog.CloseButton class="ml-auto h-full">
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

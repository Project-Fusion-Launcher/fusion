import { X } from "lucide-solid";
import type { GameSource, DownloadItem as Item } from "../../models/types";
import { bytesToSize } from "../../util/string";
import { IconButton } from "@repo/ui";
import { Show } from "solid-js";
import { Progress } from "@kobalte/core/progress";

interface DownloadItemProps {
  item: Item;
  onRemove?: (gameId: string, gameSource: GameSource) => void;
  noBorder?: boolean;
}

const DownloadItem = (props: DownloadItemProps) => {
  return (
    <div
      class="border-border h-136 text-primary relative flex w-full items-center gap-16 rounded border p-12"
      classList={{ "border overflow-hidden": !props.noBorder }}
    >
      <img
        class="h-full rounded-lg object-cover"
        src={
          "https://cdn.cloudflare.steamstatic.com/steam/apps/12110/library_600x900_2x.jpg?t=1573772047"
        }
        loading="lazy"
      />
      <div class="flex flex-col justify-center gap-8">
        <span class="h-min text-base font-medium">{props.item.gameTitle}</span>
        <span class="h-min text-sm font-medium">
          {props.item.downloaded
            ? bytesToSize(props.item.downloaded)
            : "0 Bytes"}{" "}
          <span class="text-secondary">
            / {bytesToSize(props.item.downloadSize)}
          </span>
        </span>
      </div>
      <Show when={props.onRemove}>
        <div class="ml-auto">
          <IconButton
            variant="ghost"
            size="lg"
            onClick={() =>
              props.onRemove &&
              props.onRemove(props.item.gameId, props.item.gameSource)
            }
          >
            <X />
          </IconButton>
        </div>
      </Show>
      <Progress
        value={props.item.downloaded}
        minValue={0}
        maxValue={props.item.downloadSize}
      >
        <Progress.Track class="bg-secondary absolute bottom-0 left-0 h-[2px] w-full">
          <Progress.Fill class="bg-accent h-full w-[var(--kb-progress-fill-width)] transition-all" />
        </Progress.Track>
      </Progress>
    </div>
  );
};

export default DownloadItem;

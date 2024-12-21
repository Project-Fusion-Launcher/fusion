import { X } from "lucide-solid";
import type { GameSource, DownloadItem as Item } from "../../models/types";
import { bytesToSize } from "../../util/string";
import { IconButton } from "@repo/ui";
import { Show } from "solid-js";

interface DownloadItemProps {
  item: Item;
  onRemove?: (gameId: string, gameSource: GameSource) => void;
}

const DownloadItem = (props: DownloadItemProps) => {
  return (
    <div class="border-border h-136 text-primary relative flex w-full items-center gap-16 overflow-hidden rounded border p-12">
      <img
        class="h-full rounded object-cover"
        src={
          "https://cdn.cloudflare.steamstatic.com/steam/apps/12110/library_600x900_2x.jpg?t=1573772047"
        }
        loading="lazy"
      />
      <div class="flex flex-col justify-center gap-8">
        <span class="text-md h-min font-medium">{props.item.gameTitle}</span>
        <span class="h-min text-base font-medium">
          {props.item.downloaded ? bytesToSize(props.item.downloaded) : 0}{" "}
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
    </div>
  );
};

export default DownloadItem;

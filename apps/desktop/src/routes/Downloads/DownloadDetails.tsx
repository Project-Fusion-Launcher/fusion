import { Show } from "solid-js";
import type { DownloadItem } from "../../models/types";
import DownloadItemComponent from "./DownloadItem";

interface DownloadDetailsProps {
  item?: DownloadItem;
}

const DownloadDetails = (props: DownloadDetailsProps) => {
  return (
    <>
      <div class="h-136 min-h-136 border-border text-primary w-full border-b">
        Details go here
      </div>
      <Show when={props.item}>
        <div class="border-border text-primary border-b">
          <DownloadItemComponent item={props.item as DownloadItem} noBorder />
        </div>
      </Show>
    </>
  );
};

export default DownloadDetails;

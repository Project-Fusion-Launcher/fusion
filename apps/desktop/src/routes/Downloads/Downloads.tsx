import Header from "../../components/Header";
import DownloadItem from "./DownloadItem";

const Downloads = () => {
  return (
    <>
      <Header title="Downloads" hideSearch />
      <div class="h-192 border-border text-primary w-full border-b">
        Details go here
      </div>
      <div class="flex flex-col gap-40 overflow-y-auto p-40">
        <div class="flex flex-col gap-40">
          <span class="text-primary text-lg font-medium">Up Next (3)</span>
          <div class="flex flex-col gap-16">
            <DownloadItem />
            <DownloadItem />
            <DownloadItem />
          </div>
        </div>
        <div class="">
          <span class="text-primary text-lg font-medium">Completed (0)</span>
        </div>
      </div>
    </>
  );
};

export default Downloads;

const DownloadItem = () => {
  return (
    <div class="border-border h-136 text-primary relative flex w-full gap-16 overflow-hidden rounded border p-12">
      <img
        class="h-full rounded object-cover"
        src={
          "https://cdn.cloudflare.steamstatic.com/steam/apps/12110/library_600x900_2x.jpg?t=1573772047"
        }
        loading="lazy"
      />
      <div class="flex flex-col justify-center gap-8">
        <span class="text-md h-min font-medium">
          Grand Theft Auto: Vice City
        </span>
        <span class="h-min text-base font-medium">
          238.6 MB <span class="text-secondary">/ 1.2 GB</span>
        </span>
      </div>
    </div>
  );
};

export default DownloadItem;

export function capitalizeFirstLetter(string: string) {
  return string.charAt(0).toUpperCase() + string.slice(1);
}

export function bytesToSize(bytes: number | null | undefined) {
  const sizes = ["Bytes", "KB", "MB", "GB", "TB"];

  if (bytes === 0 || bytes === null || bytes === undefined) return "Unknown";

  const i = Math.floor(Math.log(bytes) / Math.log(1024));
  return (bytes / Math.pow(1024, i)).toFixed(2) + " " + sizes[i];
}

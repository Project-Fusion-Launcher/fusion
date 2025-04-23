export function capitalizeFirstLetter(string: string) {
  return string.charAt(0).toUpperCase() + string.slice(1);
}

export function bytesToSize(bytes: number | null | undefined) {
  const sizes = ["Bytes", "KB", "MB", "GB", "TB"];
  const base = 1000;

  if (bytes === 0 || bytes === null || bytes === undefined) return "Unknown";

  const i = Math.floor(Math.log(bytes) / Math.log(base));
  return (bytes / Math.pow(base, i)).toFixed(2) + " " + sizes[i];
}

export function parseSearchParam<T>(value: string | string[] | undefined) {
  if (Array.isArray(value)) {
    return (value[0] as T) || undefined;
  }
  return (value as T) || undefined;
}

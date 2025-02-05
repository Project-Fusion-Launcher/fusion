import type { ClassValue } from "tailwind-variants";
import { clsx } from "clsx";

export function cn(...inputs: ClassValue[]) {
  // This should ideally include tailwind-merge
  return clsx(inputs);
}

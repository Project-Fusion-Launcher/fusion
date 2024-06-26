// tailwind config is required for editor support

import type { Config } from "tailwindcss";
import sharedConfig from "@repo/config-tailwind";

const config: Pick<Config, "prefix" | "presets" | "content"> = {
  content: ["./src/**/*.tsx"],
  prefix: "icons-",
  presets: [sharedConfig],
};

export default config;

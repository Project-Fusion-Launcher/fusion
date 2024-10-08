// tailwind config is required for editor support

import type { Config } from "tailwindcss";
import sharedConfig from "@repo/config-tailwind";

const config: Pick<Config, "content" | "presets"> = {
  content: ["./src/**/*.tsx", "./index.html", "../../packages/ui/src/**/*.tsx"],
  presets: [sharedConfig],
};

export default config;

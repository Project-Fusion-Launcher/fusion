// tailwind config is required for editor support

import type { Config } from "tailwindcss";
import sharedConfig from "@repo/tailwind-config";

const config: Pick<Config, "prefix" | "presets" | "content"> = {
    content: ["./**/*.tsx"],
    prefix: "ui-",
    presets: [sharedConfig],
};

export default config;

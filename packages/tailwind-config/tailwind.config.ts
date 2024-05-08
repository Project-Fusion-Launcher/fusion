import type { Config } from "tailwindcss";
import kobaltePlugin from "@kobalte/tailwindcss";

// We want each package to be responsible for its own content.
/** @type {import('tailwindcss').Config} */
const config: Omit<Config, "content"> = {
  theme: {
    colors: {
      transparent: "transparent",
      current: "currentColor",
      border: "#373737",
      background: "#0A0A0A",
      primary: {
        DEFAULT: "#BAB6BE",
      },
      secondary: {
        DEFAULT: "#726F76",
      },
      accent: {
        DEFAULT: "#874295",
      },
    },
    spacing: {
      "48": "3rem",
      "72": "4.5rem",
    },
    borderWidth: {
      DEFAULT: "1px",
      "0": "0",
    },
  },
  plugins: [kobaltePlugin],
  darkMode: "selector",
};
export default config;

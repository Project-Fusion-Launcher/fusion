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
      "8": "0.5rem",
      "28": "1.75rem",
      "32": "2rem",
      "40": "2.5rem",
      "44": "2.75rem",
      "48": "3rem",
      "52": "3.25rem",
      "72": "4.5rem",
      "136": "8.5rem",
    },
    borderWidth: {
      DEFAULT: "1px",
      "0": "0",
      sm: "1px",
      md: "2px",
    },
  },
  plugins: [kobaltePlugin],
  darkMode: "selector",
};
export default config;

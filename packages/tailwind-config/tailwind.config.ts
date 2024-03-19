import type { Config } from "tailwindcss";
import kobaltePlugin from "@kobalte/tailwindcss";

// We want each package to be responsible for its own content.
/** @type {import('tailwindcss').Config} */
const config: Omit<Config, "content"> = {
    theme: {
        extend: {
            backgroundImage: {
                "glow-conic":
                    "conic-gradient(from 180deg at 50% 50%, #2a8af6 0deg, #a853ba 180deg, #e92a67 360deg)",
            },
        },
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

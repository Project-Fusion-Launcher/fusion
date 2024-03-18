import type { Config } from "tailwindcss";
import kobaltePlugin from "@kobalte/tailwindcss";

// We want each package to be responsible for its own content.
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
            primary: "#0A0A0A",
            secondary: "#373737",
            "text-primary": "#BAB6BE",
            "text-secondary": "#726F76",
            accent: "#874295",
        },
    },
    plugins: [kobaltePlugin],
    darkMode: "selector",
};
export default config;

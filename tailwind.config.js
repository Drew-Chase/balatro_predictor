import {heroui} from "@heroui/react";

/** @type {import('tailwindcss').Config} */
export default {
    content: [
        "./index.html",
        "./src/**/*.{js,ts,jsx,tsx}",
        "./node_modules/@heroui/theme/dist/**/*.{js,ts,jsx,tsx}"
    ],
    theme: {
        extend: {},
    },
    darkMode: "class",
    plugins: [heroui({
        themes: {
            light: {
                colors: {
                    primary: {
                        DEFAULT: "#2f4cef",
                        foreground: "#fff",
                    },
                    secondary: "#2b2b2b",
                    background: "#ffffff",

                }
            },
            dark: {
                colors: {
                    primary: "#3273ff",
                    secondary: "#eaeaea",
                    background: "#0e0e0e",
                }
            },
        }
    })]
}
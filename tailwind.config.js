/** @type {import('tailwindcss').Config} */
export default {
  darkMode: "class",
  content: ["./index.html", "./src/**/*.{ts,tsx}"],
  theme: {
    extend: {
      colors: {
        savage: {
          bg: "#0b0d12",
          panel: "#12151c",
          accent: "#ff4d5e",
          accent2: "#7c5cff",
          good: "#3ddc97",
          warn: "#ffb020",
        },
      },
      backdropBlur: {
        xs: "2px",
      },
    },
  },
  plugins: [],
};

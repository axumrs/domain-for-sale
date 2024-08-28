/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{vue,js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        price: "#5ce09e",
        primary: "#fbf8f3",
        dark: "#1d1025",
      },
    },
  },
  plugins: [],
};

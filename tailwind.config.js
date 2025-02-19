/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {},
  },
  plugins: [],
  presets: [
    require('@interstate/tailwind-preset')({
      useTypography: false,
      useForms: false,
    }),
  ],
};

/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "jit",
  content: ["src/**/*.rs", "index.html"],
  theme: {
    extend: {},
  },
  plugins: [
    require("@tailwindcss/forms")({
      strategy: 'base', // only generate global styles
      strategy: 'class', // only generate classes
    }),
  ],
}


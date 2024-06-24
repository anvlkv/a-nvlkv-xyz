module.exports = {
  content: {
    files: ["src/**/*.rs"],
  },
  darkMode: [
    "variant",
    [
      "@media (prefers-color-scheme: dark) { &:not(.theme-light *) }",
      "&:is(.theme-dark *)",
    ],
  ],
  theme: {
    extend: {},
  },
  variants: {
    extend: {},
  },
  plugins: [],
};

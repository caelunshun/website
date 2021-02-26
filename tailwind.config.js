const production = !process.env.ROLLUP_WATCH;

module.exports = {
  future: {
    removeDeprecatedGapUtilities: true,
    purgeLayersByDefault: true,
  },
  plugins: [],
  purge: {
    content: ["./src/**/*.svelte"],
    enabled: production,
  },
  theme: {
    fontFamily: {
      sans: ["Roboto"],
      display: ["Roboto", "sans-serif"],
      body: ["Roboto", "sans-serif"],
    },
    extend: {
      colors: {
        feather: {
          light: "#BADEDA",
          dark: "#005E66",
          accent: "#E29578",
        },
      },
    },
  },
}

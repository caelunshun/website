module.exports = {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
    "postcss-font-magician": {
      variants: {
        "Roboto": {
            "300": [],
            "400": [],
            "700": []
        }
    },
    foundries: ['google']
    }
  },
}

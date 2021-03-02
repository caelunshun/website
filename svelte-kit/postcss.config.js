const production = !process.env.ROLLUP_WATCH;

module.exports = {
  plugins: [
    // require("tailwindcss"),
    // require("autoprefixer"),
    // require("postcss-font-magician")({
    //   variants: {
    //     "Roboto": {
    //       "300": [],
    //       "400": [],
    //       "700": []
    //     }
    //   },
    //   foundries: ["google"]
    // }),
    // ...(production
    //   ? [require("autoprefixer"), require("cssnano")({ preset: "default" })]
    //   : []),
  ],
};
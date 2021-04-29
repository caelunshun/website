const { tailwindExtractor } = require("tailwindcss/lib/lib/purgeUnusedStyles");

module.exports = {
  darkMode: "class",
  purge: {
    content: [
      "./src/**/*.html",
      "./src/**/*.svelte",
    ],
    options: {
      defaultExtractor: (content) => [
        // This is an internal Tailwind function that we're not supposed to be allowed to use
        // So if this stops working, please open an issue at
        // https://github.com/babichjacob/sapper-postcss-template/issues
        // rather than bothering Tailwind Labs about it
        ...tailwindExtractor(content),
        // Match Svelte class: directives (https://github.com/tailwindlabs/tailwindcss/discussions/1731)
        ...[...content.matchAll(/(?:class:)*([\w\d-/:%.]+)/gm)].map(([_match, group, ..._rest]) => group),
      ],
      keyframes: true,
    },
  },
  theme: {
    fontFamily: {
      sans: ["Roboto"],
      display: ["Roboto", "sans-serif"],
      body: ["Roboto", "sans-serif"],
      mono: ["ui-monospace", "SFMono-Regular", "Menlo", "Monaco", "Consolas", "Liberation Mono", "Courier New", "monospace"],
    },
    extend: {
      colors: {
        feather: {
          light: "#BADEDA",
          dark: "#57804d",
          accent: "#E29578",
          a: "#006D77",
          b: "#83c58e",
          c: "#EDF6F9",
          d: "#FFDDD2",
          e: "#E29578",
        },
      },
      typography: (theme) => ({
        dark: {
          css: {
            color: theme("colors.gray.300"),
            a: {
              color: theme("colors.gray.200"),
            },
            h1: {
              color: theme("colors.gray.50"),
            },
            h2: {
              color: theme("colors.gray.50"),
            },
            h3: {
              color: theme("colors.gray.50"),
            },
            h4: {
              color: theme("colors.gray.50"),
            },
            h5: {
              color: theme("colors.gray.50"),
            },
            h6: {
              color: theme("colors.gray.50"),
            },
            strong: {
              color: theme("colors.gray.50"),
            },
            code: {
              color: theme("colors.gray.50"),
            },
            figcaption: {
              color: theme("colors.gray.50"),
            },
            p: {
              color: theme("colors.gray.50"),
            },
            blockquote: {
              color: theme("colors.gray.50"),
            },
          },
        },
      }),
    },
    minWidth: {
      "1/2": "50%",
      "70p": "70%",
    },
  },
  variants: {
    extend: {
      backgroundColor: ["active"],
      ringWidth: ["active"],
      boxShadow: ["active"],
      typography: ["dark"],
      textColor: ["active"],
    },
  },
  plugins: [
    require("@tailwindcss/typography")
  ],
};

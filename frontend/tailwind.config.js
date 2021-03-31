const { tailwindExtractor } = require("tailwindcss/lib/lib/purgeUnusedStyles");

module.exports = {
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
					dark: "#005E66",
					accent: "#E29578",
				},
			},
		},
		minWidth: {
			"1/2": "50%",
			"70p": "70%",
		},
		maxWidth: {
			47: "47%",
			22: "22rem",
		},
	},
	variants: {
		extend: {},
	},
	plugins: [
		require("@tailwindcss/typography")
	],
};

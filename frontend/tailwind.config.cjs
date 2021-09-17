const { tailwindExtractor } = require("tailwindcss/lib/lib/purgeUnusedStyles");

module.exports = {
    darkMode: "class",
    mode: "jit",
    purge: {
        content: ["./src/**/*.{html,js,svelte,ts}"],
        options: {
            defaultExtractor: (content) => [
                // If this stops working, please open an issue at https://github.com/svelte-add/tailwindcss/issues rather than bothering Tailwind Labs about it
                ...tailwindExtractor(content),
                // Match Svelte class: directives (https://github.com/tailwindlabs/tailwindcss/discussions/1731)
                ...[...content.matchAll(/(?:class:)*([\w\d-/:%.]+)/gm)].map(
                    ([_match, group, ..._rest]) => group
                )
            ]
        }
    },
    theme: {
        fontFamily: {
            sans: ["Roboto"],
            display: ["Roboto", "sans-serif"],
            body: ["Roboto", "sans-serif"],
            mono: [
                "ui-monospace",
                "SFMono-Regular",
                "Menlo",
                "Monaco",
                "Consolas",
                "Liberation Mono",
                "Courier New",
                "monospace"
            ]
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
                    e: "#E29578"
                },
                nether: {
                    lava: "#D0520C",
                    glowstone: "#E8AC46",
                    netherbrick: "#401616",
                    netherrack: {
                        dark: "#703131",
                        DEFAULT: "#824141"
                    },
                    blackstone: "#160f10"
                },
                achievement: {
                    border: "rgb(85,85,85)",
                    background: "rgb(33,33,33)"
                },
                github: "#161B22"
            },
            blur: {
                xs: "2px"
            },
            height: {
                bigscreen: "80vh",
                smallscreen: "50vh"
            },
            typography: (theme) => ({
                DEFAULT: {
                    css: {
                        "code::before": {
                            content: '""'
                        },
                        "code::after": {
                            content: '""'
                        },
                        a: {
                            textDecoration: "none",
                            borderBottom: "2px solid",
                            borderColor: theme("colors.feather.accent"),
                            fontWeight: "bold"
                        }
                    }
                },
                sm: {
                    css: {
                        code: {
                            fontWeight: "600"
                        }
                    }
                },
                lg: {
                    css: {
                        code: {
                            fontWeight: "600"
                        }
                    }
                },
                xl: {
                    css: {
                        code: {
                            fontWeight: "600"
                        }
                    }
                },
                "2xl": {
                    css: {
                        code: {
                            fontWeight: "600"
                        }
                    }
                },
                dark: {
                    css: {
                        color: theme("colors.gray.300"),
                        a: {
                            color: theme("colors.gray.200")
                        },
                        h1: {
                            color: theme("colors.gray.50")
                        },
                        h2: {
                            color: theme("colors.gray.50")
                        },
                        h3: {
                            color: theme("colors.gray.50")
                        },
                        h4: {
                            color: theme("colors.gray.50")
                        },
                        h5: {
                            color: theme("colors.gray.50")
                        },
                        h6: {
                            color: theme("colors.gray.50")
                        },
                        strong: {
                            color: theme("colors.gray.50")
                        },
                        code: {
                            color: theme("colors.gray.50")
                        },
                        figcaption: {
                            color: theme("colors.gray.50")
                        },
                        p: {
                            color: theme("colors.gray.50")
                        },
                        blockquote: {
                            color: theme("colors.gray.50")
                        }
                    }
                }
            })
        },
        minWidth: {
            "1/2": "50%",
            "70p": "70%"
        },
        maxHeight: {
            120: "30rem"
        }
    },
    variants: {
        extend: {
            backgroundColor: ["active"],
            ringWidth: ["active"],
            boxShadow: ["active"],
            typography: ["dark"],
            textColor: ["active"],
            brightness: ["dark"]
        }
    },
    plugins: [
        require("@tailwindcss/typography"),
        require("@tailwindcss/forms")({ strategy: "class" })
    ]
};

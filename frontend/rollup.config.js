import path from "path";
import { spawn } from "child_process";
import { performance } from "perf_hooks";
import resolve from "@rollup/plugin-node-resolve";
import replace from "@rollup/plugin-replace";
import commonjs from "@rollup/plugin-commonjs";
import svelte from "rollup-plugin-svelte";
import babel from "@rollup/plugin-babel";
import colors from "kleur";
import { terser } from "rollup-plugin-terser";
import config from "sapper/config/rollup";
import alias from "@rollup/plugin-alias";
import svelteSVG from "rollup-plugin-svelte-svg";
import json from "@rollup/plugin-json";
import typescript from "@rollup/plugin-typescript";
import pkg from "./package.json";

const { createPreprocessors } = require("./svelte.config.js");

const mode = process.env.NODE_ENV;
const dev = mode === "development";
const sourcemap = dev ? "inline" : false;
const legacy = !!process.env.SAPPER_LEGACY_BUILD;

const preprocess = createPreprocessors({ sourceMap: !!sourcemap });

// Changes in these files will trigger a rebuild of the global CSS
const globalCSSWatchFiles = ["postcss.config.js", "tailwind.config.js", "src/global.pcss"];

const onwarn = (warning, onwarnc) => (warning.code === "MISSING_EXPORT" && /'preload'/.test(warning.message))
    || (warning.code === "CIRCULAR_DEPENDENCY" && /[/\\]@sapper[/\\]/.test(warning.message))
    || (warning.code === "THIS_IS_UNDEFINED")
    || onwarnc(warning);

const projectRootDir = path.resolve(__dirname);
const aliases = alias({
  entries: [
    {
      find: "$src",
      replacement: path.resolve(projectRootDir, "src"),
    },
    {
      find: "$components",
      replacement: path.resolve(projectRootDir, "src/components"),
    },
    {
      find: "$static",
      replacement: path.resolve(projectRootDir, "static"),
    },
    {
      find: "$assets",
      replacement: path.resolve(projectRootDir, "src/assets"),
    },
    {
      find: "$stores",
      replacement: path.resolve(projectRootDir, "src/stores"),
    },
  ],
});

const FEAHTER_API_CLIENT = process.env.FEATHER_CLIENT_API || "http://localhost:4000";
const FEATHER_API_SERVER = process.env.FEATHER_SERVER_API || FEAHTER_API_CLIENT;
const GITHUB_IDENTITY = process.env.GITHUB_IDENTITY || "https://github.com/login/oauth/authorize?client_id=fd248e478ca35447716b";

export default {
  client: {
    input: config.client.input().replace(/\.js$/, ".ts"),
    output: { ...config.client.output(), sourcemap },
    plugins: [
      aliases,
      svelteSVG({ dev }),
      json(),
      replace({
        preventAssignment: true,
        values: {
          "process.browser": true,
          "process.env.NODE_ENV": JSON.stringify(mode),
          "process.env.FEATHER_API": FEAHTER_API_CLIENT,
          "process.env.GITHUB_IDENTITY": GITHUB_IDENTITY,
        },
      }),
      svelte({
        compilerOptions: {
          dev,
          hydratable: true,
        },
        emitCss: true,
        preprocess,
      }),
      resolve({
        browser: true,
        dedupe: ["svelte"],
      }),
      commonjs({
        sourceMap: !!sourcemap,
      }),

      legacy && babel({
        extensions: [".js", ".mjs", ".html", ".svelte"],
        babelHelpers: "runtime",
        exclude: ["node_modules/@babel/**"],
        presets: [
          ["@babel/preset-env", {
            targets: "> 0.25%, not dead",
          }],
        ],
        plugins: [
          "@babel/plugin-syntax-dynamic-import",
          ["@babel/plugin-transform-runtime", {
            useESModules: true,
          }],
        ],
      }),

      !dev && terser({
        module: true,
      }),

      (() => {
        let builder;
        let rebuildNeeded = false;

        const buildGlobalCSS = () => {
          if (builder) {
            rebuildNeeded = true;
            return;
          }
          rebuildNeeded = false;
          const start = performance.now();

          try {
            builder = spawn("node", ["--experimental-modules", "--unhandled-rejections=strict", "build-global-css.mjs", sourcemap]);
            builder.stdout.pipe(process.stdout);
            builder.stderr.pipe(process.stderr);

            builder.on("close", (code) => {
              if (code === 0) {
                const elapsed = parseInt(performance.now() - start, 10);
                console.log(`${colors.bold().green("✔ global css")} (src/global.pcss → static/global.css${sourcemap === true ? " + static/global.css.map" : ""}) ${colors.gray(`(${elapsed}ms)`)}`);
              } else if (code !== null) {
                if (dev) {
                  console.error(`global css builder exited with code ${code}`);
                  console.log(colors.bold().red("✗ global css"));
                } else {
                  throw new Error(`global css builder exited with code ${code}`);
                }
              }

              builder = undefined;

              if (rebuildNeeded) {
                console.log(`\n${colors.bold().italic().cyan("something")} changed. rebuilding...`);
                buildGlobalCSS();
              }
            });
          } catch (err) {
            console.log(colors.bold().red("✗ global css"));
            console.error(err);
          }
        };

        return {
          name: "build-global-css",
          buildStart() {
            buildGlobalCSS();
            globalCSSWatchFiles.forEach((file) => this.addWatchFile(file));
          },
          generateBundle: buildGlobalCSS,
        };
      })(),
      typescript({
        noEmitOnError: !dev,
        sourceMap: !!sourcemap,
      }),
    ],

    preserveEntrySignatures: false,
    onwarn,
  },

  server: {
    input: config.server.input().server.replace(/\.js$/, ".ts"),
    output: { ...config.server.output(), sourcemap },
    plugins: [
      aliases,
      svelteSVG({ generate: "ssr", dev }),
      json(),
      replace({
        preventAssignment: true,
        values: {
          "process.browser": false,
          "process.env.NODE_ENV": JSON.stringify(mode),
          "process.env.FEATHER_API": FEATHER_API_SERVER,
          "process.env.GITHUB_IDENTITY": GITHUB_IDENTITY,
        },
      }),
      svelte({
        compilerOptions: {
          dev,
          generate: "ssr",
        },
        preprocess,
      }),
      resolve({
        dedupe: ["svelte"],
      }),
      commonjs({
        sourceMap: !!sourcemap,
      }),
      typescript({
        noEmitOnError: !dev,
        sourceMap: !!sourcemap,
      }),
    ],
    external: Object.keys(pkg.dependencies).concat(
      require("module").builtinModules || Object.keys(process.binding("natives")), // eslint-disable-line global-require
    ),
    preserveEntrySignatures: "strict",
    onwarn,
  },

  serviceworker: {
    input: config.serviceworker.input().replace(/\.js$/, ".ts"),
    output: { ...config.serviceworker.output(), sourcemap },
    plugins: [
      resolve(),
      replace({
        preventAssignment: true,
        values: {
          "process.browser": true,
          "process.env.NODE_ENV": JSON.stringify(mode),
          "process.env.FEATHER_API": FEAHTER_API_CLIENT,
        },
      }),
      commonjs({
        sourceMap: !!sourcemap,
      }),
      !dev && terser(),
      typescript({
        noEmitOnError: !dev,
        sourceMap: !!sourcemap,
      }),
    ],

    preserveEntrySignatures: false,
    onwarn,
  },
};

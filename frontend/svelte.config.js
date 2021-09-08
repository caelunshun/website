import preprocess from "svelte-preprocess";
import svelteSVG from "vite-plugin-svelte-svg";
import adapterCfw from "@sveltejs/adapter-cloudflare-workers";

/** @type {import('@sveltejs/kit').Config} */
const config = {
    // Consult https://github.com/sveltejs/svelte-preprocess
    // for more information about preprocessors
    preprocess: [
        preprocess({
            defaults: {
                style: "postcss"
            },
            postcss: true
        })
    ],

    kit: {
        // hydrate the <div id="svelte"> element in src/app.html
        files: {
            assets: "src/static"
        },
        target: "#svelte",
        vite: {
            plugins: [svelteSVG()]
        },
        adapter: adapterCfw()
    }
};

export default config;

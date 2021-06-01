<script context="module" lang="ts">
    import * as docscache from '$lib/stores/docscache';
    import { API_BASE_URL } from "$lib/env"; 

    export async function load({ params: { slug } }: {params: {slug: string[]}}) {
        let slugpath = slug.join("/");
        if(docscache.has(slugpath)) {
            return { html: docscache.get(slugpath) }
        } else {
            const response = await this.fetch(`${API_BASE_URL}/docs/page/${encodeURI(slugpath)}?base_url=${encodeURIComponent("/")}`)
            let markdown_html = await response.text();
            markdown_html = markdown_html.replaceAll("https://raw.githubusercontent.com/Defman/feather/Docs/docs/src/", "/docs/");
            docscache.set(slugpath, markdown_html);
            return { html: markdown_html };
        }
    }
</script>

<svelte:head>
    <title>Docs | Feather</title>
</svelte:head>

<script lang="ts">
    export let html: string;
</script>

{@html html}

<style>

</style>
<script context="module" lang="ts">
    import * as docscache from "$lib/stores/docscache";
    import { docheadings } from "$lib/stores/local";
    import { API_BASE_URL } from "$lib/env";
    import type { DocResponse } from "$lib/types";
    import type { Load } from "@sveltejs/kit";
    export const load: Load = async ({
        page: {
            params: { slug }
        },
        fetch
    }) => {
        if (docscache.has(slug)) {
            const cached: DocResponse = docscache.get(slug);
            docheadings.set(cached.topics);
            return { props: { html: cached.html } };
        } else {
            const response_fetch = await fetch(
                `${API_BASE_URL}/docs/page/${encodeURI(slug)}?base_url=${encodeURIComponent("/")}`
            );
            let response: DocResponse = await response_fetch.json();
            let markdown_html = response.html;
            markdown_html = markdown_html.replace(
                /https\:\/\/raw\.githubusercontent\.com\/Defman\/feather\/Docs\/docs\/src\//g,
                "/docs/"
            );
            docscache.set(slug, response);
            docheadings.set(response.topics);
            return { props: { html: markdown_html } };
        }
    };
</script>

<script lang="ts">
    export let html: string;
</script>

<svelte:head>
    <title>Docs | Feather</title>
</svelte:head>

{@html html}

<style>
</style>

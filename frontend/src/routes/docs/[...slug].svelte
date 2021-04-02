<script context="module" lang="ts">
    import * as docscache from '$stores/docscache';

    export async function preload({ params: { slug } }: {params: {slug: string[]}}) {
        let slugpath = slug.join("/");
        if(docscache.has(slugpath)) {
            return { html: docscache.get(slugpath) }
        } else {
            const response = await this.fetch(`process.env.FEATHER_API/docs/page/${encodeURI(slugpath)}`)
            let markdown_html = await response.text();
            markdown_html = markdown_html.replaceAll("http://localhost:3000", "/docs/");
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
<script context="module">
    import { API_BASE_URL } from "$lib/env"; 

    export async function preload() {
        const response = await fetch(`${API_BASE_URL}/association/articles-of-association`)
        let markdown_html = await response.text();
        return { html: markdown_html };
    }
</script>

<script>
    export let html
</script>

<div class="mx-4 md:w-10/12 md:mx-auto prose dark:prose-dark xl:prose-xl markdown my-10 md:my-24">
    {@html html}
</div>


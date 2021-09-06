<script context="module">
    import { API_BASE_URL } from "$lib/env";
    export async function load() {
        const response = await fetch(`${API_BASE_URL}/association/budget`);
        let markdown_html = await response.text();
        return { props: { html: markdown_html } };
    }
</script>

<script>
    export let html;
</script>

<div class="mx-4 md:w-10/12 md:mx-auto prose dark:prose-dark xl:prose-xl markdown my-10 md:my-24">
    {@html html}
</div>
